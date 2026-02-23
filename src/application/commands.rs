use crate::cli::{Cli, Commands};
use crate::core::changelog::{parse_header_format, ChangelogDocument, Release};
use crate::core::git::{latest_semver_tag, CommitRange, Git2Repository, RepositoryApi};
use crate::core::notes::notes_from_commits;
use crate::error::{ChangelogError, Result};
use clap::Parser;
use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn execute() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { file, format } => command_new(&file, &format),
        Commands::Validate { file, strict } => command_validate(&file, strict),
        Commands::Generate {
            file,
            since,
            until,
            specific,
            milestone,
            github,
            template,
            output,
            map,
        } => {
            let args = GenerateArgs {
                file,
                since,
                until,
                specific,
                milestone,
                github,
                template,
                output,
                map,
            };
            command_generate(args).await
        }
        Commands::Release {
            version,
            bump,
            file,
            header,
            override_existing,
        } => {
            let args = ReleaseArgs {
                version,
                bump,
                file,
                header,
                override_existing,
            };
            command_release(args).await
        }
        Commands::Show {
            file,
            version,
            range,
            converge,
        } => command_show(&file, version.as_deref(), range.as_deref(), converge),
        Commands::Remove { version, file, yes } => command_remove(&file, &version, yes),
    }
}

fn command_new(file: &Path, format: &str) -> Result<()> {
    if format != "markdown" {
        return Err(ChangelogError::InvalidArguments(format!(
            "unsupported format '{}'; only 'markdown' is currently supported",
            format
        )));
    }

    let doc = ChangelogDocument::scaffold();
    write_file(file, &doc.to_markdown())?;
    println!("Created {}", file.display());
    Ok(())
}

fn command_validate(file: &Path, strict: bool) -> Result<()> {
    let content = read_or_create_scaffold(file)?;
    let document = ChangelogDocument::parse(&content).map_err(|issue| {
        ChangelogError::InvalidArguments(format!("validation failed: {}", issue.message()))
    })?;

    document.validate(strict).map_err(|issue| {
        ChangelogError::InvalidArguments(format!("validation failed: {}", issue.message()))
    })?;

    println!("{} is valid", file.display());
    Ok(())
}

#[derive(Debug)]
struct GenerateArgs {
    file: PathBuf,
    since: Option<String>,
    until: Option<String>,
    specific: Option<String>,
    milestone: Option<String>,
    github: bool,
    template: Option<PathBuf>,
    output: Option<PathBuf>,
    map: Option<PathBuf>,
}

async fn command_generate(args: GenerateArgs) -> Result<()> {
    if args.milestone.is_some() && !args.github {
        return Err(ChangelogError::InvalidArguments(
            "--milestone requires --github".to_string(),
        ));
    }

    if args.milestone.is_some() {
        return Err(ChangelogError::UnsupportedFeature(
            "milestone mode requires GitHub API integration, which is not yet enabled".to_string(),
        ));
    }

    let _existing_doc = load_or_scaffold(&args.file)?;

    let repo = Git2Repository::open(".").map_err(|e| ChangelogError::GitError(e.to_string()))?;
    let range = CommitRange {
        since: args.since,
        until: args.until,
        specific: args.specific,
    };
    let commits = repo
        .list_commits(&range)
        .map_err(|e| ChangelogError::GitError(e.to_string()))?;

    let mapping = load_type_mapping(args.map.as_deref())?;
    let grouped = notes_from_commits(&commits, &mapping);

    let markdown = render_generated_sections(grouped, args.template.as_deref())
        .map_err(|e| ChangelogError::TemplateError(e.to_string()))?;

    if let Some(output) = args.output {
        write_file(&output, &markdown)?;
        println!("Wrote generated notes to {}", output.display());
    } else {
        println!("{}", markdown);
    }

    Ok(())
}

#[derive(Debug)]
struct ReleaseArgs {
    version: Option<String>,
    bump: Option<String>,
    file: PathBuf,
    header: String,
    override_existing: bool,
}

async fn command_release(args: ReleaseArgs) -> Result<()> {
    let mut document = load_or_scaffold(&args.file)?;

    let new_version = match (args.version, args.bump.as_deref()) {
        (Some(raw), None) => {
            Version::parse(&raw).map_err(|_| ChangelogError::VersionParseError(raw.to_string()))?
        }
        (None, Some(bump)) => {
            let base = document
                .releases
                .iter()
                .map(|r| r.version.clone())
                .max()
                .unwrap_or_else(|| Version::new(0, 0, 0));
            bump_version(base, bump)?
        }
        _ => {
            return Err(ChangelogError::InvalidArguments(
                "use exactly one of --version or --bump".to_string(),
            ));
        }
    };

    let repo = Git2Repository::open(".").map_err(|e| ChangelogError::GitError(e.to_string()))?;
    let tags = repo
        .list_tags()
        .map_err(|e| ChangelogError::GitError(e.to_string()))?;

    let range = CommitRange {
        since: latest_semver_tag(&tags),
        until: None,
        specific: None,
    };

    let commits = repo
        .list_commits(&range)
        .map_err(|e| ChangelogError::GitError(e.to_string()))?;

    let mapping = load_type_mapping(None)?;
    let grouped = notes_from_commits(&commits, &mapping);

    let mut release = Release::new(new_version);
    release.header = parse_header_format(&args.header);
    release.sections = grouped;

    document
        .upsert_release(release, args.override_existing)
        .map_err(ChangelogError::InvalidArguments)?;

    document
        .validate(false)
        .map_err(|issue| ChangelogError::InvalidArguments(issue.message()))?;

    write_file(&args.file, &document.to_markdown())?;
    println!("Updated {}", args.file.display());
    Ok(())
}

fn command_show(
    file: &Path,
    version: Option<&str>,
    range: Option<&str>,
    converge: bool,
) -> Result<()> {
    let document = load_or_scaffold(file)?;

    let mut selected: Vec<Release> = if let Some(raw) = version {
        let target =
            Version::parse(raw).map_err(|_| ChangelogError::VersionParseError(raw.to_string()))?;
        document
            .releases
            .iter()
            .filter(|release| release.version == target)
            .cloned()
            .collect()
    } else if let Some(raw_range) = range {
        let (left, right) = raw_range.split_once("..").ok_or_else(|| {
            ChangelogError::InvalidArguments("--range must use '<a>..<b>' format".to_string())
        })?;

        let a = Version::parse(left)
            .map_err(|_| ChangelogError::VersionParseError(left.to_string()))?;
        let b = Version::parse(right)
            .map_err(|_| ChangelogError::VersionParseError(right.to_string()))?;
        let (low, high) = if a <= b { (a, b) } else { (b, a) };

        document
            .releases
            .iter()
            .filter(|release| release.version >= low && release.version <= high)
            .cloned()
            .collect()
    } else {
        document.releases.clone()
    };

    if selected.is_empty() {
        return Err(ChangelogError::InvalidArguments(
            "no matching releases found".to_string(),
        ));
    }

    selected.sort_by(|a, b| b.version.cmp(&a.version));

    if converge {
        let converged = converge_releases(&selected);
        println!("{}", converged.to_markdown());
    } else {
        let doc = ChangelogDocument {
            title: document.title,
            releases: selected,
        };
        println!("{}", doc.to_markdown());
    }

    Ok(())
}

fn command_remove(file: &Path, version: &str, yes: bool) -> Result<()> {
    if !yes {
        return Err(ChangelogError::InvalidArguments(
            "remove requires --yes to apply file changes".to_string(),
        ));
    }

    let mut document = load_or_scaffold(file)?;
    let target = Version::parse(version)
        .map_err(|_| ChangelogError::VersionParseError(version.to_string()))?;

    if !document.remove_version(&target) {
        return Err(ChangelogError::InvalidArguments(format!(
            "release {} was not found",
            target
        )));
    }

    write_file(file, &document.to_markdown())?;
    println!("Removed release {} from {}", target, file.display());
    Ok(())
}

fn converge_releases(releases: &[Release]) -> ChangelogDocument {
    let mut doc = ChangelogDocument::scaffold();
    let mut merged = Release::new(Version::new(0, 0, 0));
    merged.date = None;
    merged.header = crate::core::changelog::HeaderFormat::Custom("## [converged]".to_string());

    for release in releases {
        for (section, notes) in &release.sections {
            for note in notes {
                merged.add_note(section.clone(), note.clone());
            }
        }
    }

    merged.sections = crate::core::notes::dedupe_grouped_notes(&merged.sections);
    doc.releases.push(merged);
    doc
}

fn bump_version(mut version: Version, bump: &str) -> Result<Version> {
    match bump {
        "major" => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        "minor" => {
            version.minor += 1;
            version.patch = 0;
        }
        "patch" => {
            version.patch += 1;
        }
        _ => {
            return Err(ChangelogError::InvalidArguments(
                "--bump must be one of: major, minor, patch".to_string(),
            ));
        }
    }

    Ok(version)
}

fn load_or_scaffold(path: &Path) -> Result<ChangelogDocument> {
    if !path.exists() {
        return Ok(ChangelogDocument::scaffold());
    }

    let content = fs::read_to_string(path)?;
    ChangelogDocument::parse(&content).map_err(|issue| {
        ChangelogError::InvalidArguments(format!(
            "failed to parse {}: {}",
            path.display(),
            issue.message()
        ))
    })
}

fn read_or_create_scaffold(path: &Path) -> Result<String> {
    if path.exists() {
        return Ok(fs::read_to_string(path)?);
    }

    Ok(ChangelogDocument::scaffold().to_markdown())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, content)?;
    Ok(())
}

#[derive(Debug, Default, Deserialize)]
struct TypeMapping {
    types: Option<BTreeMap<String, String>>,
}

fn load_type_mapping(path: Option<&Path>) -> Result<BTreeMap<String, String>> {
    let Some(path) = path else {
        return Ok(BTreeMap::new());
    };

    let content = fs::read_to_string(path)?;
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let parsed: TypeMapping = match extension {
        "json" => serde_json::from_str(&content).map_err(|e| {
            ChangelogError::InvalidArguments(format!("invalid JSON mapping file: {}", e))
        })?,
        "toml" => toml::from_str(&content).map_err(|e| {
            ChangelogError::InvalidArguments(format!("invalid TOML mapping file: {}", e))
        })?,
        other => {
            return Err(ChangelogError::InvalidArguments(format!(
                "unsupported mapping extension '{}'; use .json or .toml",
                other
            )));
        }
    };

    Ok(parsed.types.unwrap_or_default())
}

fn render_generated_sections(
    grouped: BTreeMap<String, Vec<String>>,
    template: Option<&Path>,
) -> anyhow::Result<String> {
    if let Some(path) = template {
        let data = serde_json::json!({
            "sections": grouped
                .iter()
                .map(|(k, v)| serde_json::json!({"name": k, "notes": v}))
                .collect::<Vec<_>>()
        });
        return crate::infrastructure::templates::render(path.to_str(), &data);
    }

    let mut out = String::new();
    for (section, notes) in grouped {
        out.push_str("### ");
        out.push_str(&section);
        out.push('\n');
        for note in notes {
            out.push_str("- ");
            out.push_str(&note);
            out.push('\n');
        }
        out.push('\n');
    }

    if out.is_empty() {
        out.push_str("### Other\n- No user-facing changes detected\n");
    }

    Ok(out.trim_end().to_string() + "\n")
}
