use chrono::Utc;
use anyhow::Result;
use crate::parser::GroupedCommits;
use crate::infrastructure::github;

pub fn generate(
    grouped: GroupedCommits,
    release: Option<String>,
    github_enabled: bool,
) -> Result<String> {

    let date = Utc::now().format("%Y-%m-%d");

    let mut output = String::new();

    output.push_str("# Changelog\n\n");

    if let Some(version) = release {
        output.push_str(&format!("## [{}] - {}\n\n", version, date));
    }

    for (group, commits) in grouped {
        let group_name: String = group.to_uppercase();
        output.push_str(&format!("### {}\n", group_name));
        for commit in commits {
            output.push_str(&format!("- {}\n", commit));
        }
        output.push('\n');
    }

    if github_enabled {
        if let Ok(link) = github::generate_compare_link() {
            output.push_str(&format!("---\nCompare: {}\n", link));
        }
    }

    Ok(output)
}
