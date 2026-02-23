use crate::domain::commit::Commit;
use anyhow::{anyhow, Result};
use git2::{ObjectType, Oid, Repository, Sort};
use semver::Version;

#[derive(Debug, Clone, Default)]
pub struct CommitRange {
    pub since: Option<String>,
    pub until: Option<String>,
    pub specific: Option<String>,
}

pub trait RepositoryApi {
    fn list_commits(&self, range: &CommitRange) -> Result<Vec<Commit>>;
    fn list_tags(&self) -> Result<Vec<String>>;
}

pub struct Git2Repository {
    repo: Repository,
}

impl Git2Repository {
    pub fn open(path: &str) -> Result<Self> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    fn resolve_oid(&self, reference: &str) -> Result<Oid> {
        let object = self.repo.revparse_single(reference)?;
        Ok(object.id())
    }

    fn head_oid(&self) -> Result<Oid> {
        let head = self.repo.head()?;
        let target = head
            .target()
            .ok_or_else(|| anyhow!("HEAD does not point to a commit"))?;
        Ok(target)
    }

    fn collect_commits(&self, mut revwalk: git2::Revwalk<'_>) -> Result<Vec<Commit>> {
        revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;

        let mut items = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            let first_line = commit.summary().unwrap_or("").to_string();
            items.push(Commit {
                message: first_line,
                hash: oid.to_string(),
            });
        }

        // Deterministic oldest -> newest output for note grouping.
        items.reverse();
        Ok(items)
    }
}

impl RepositoryApi for Git2Repository {
    fn list_commits(&self, range: &CommitRange) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk()?;

        if let Some(reference) = &range.specific {
            let oid = self.resolve_oid(reference)?;
            revwalk.push(oid)?;
            let commit = self.repo.find_commit(oid)?;
            for parent in commit.parents() {
                revwalk.hide(parent.id())?;
            }
            return self.collect_commits(revwalk);
        }

        let until_oid = match &range.until {
            Some(reference) => self.resolve_oid(reference)?,
            None => self.head_oid()?,
        };
        revwalk.push(until_oid)?;

        if let Some(reference) = &range.since {
            let since_oid = self.resolve_oid(reference)?;
            revwalk.hide(since_oid)?;
        }

        self.collect_commits(revwalk)
    }

    fn list_tags(&self) -> Result<Vec<String>> {
        let mut tags = Vec::new();

        let names = self.repo.tag_names(None)?;
        for raw in names.iter().flatten() {
            if self
                .repo
                .revparse_single(raw)
                .ok()
                .map(|o| o.kind() == Some(ObjectType::Tag) || o.kind() == Some(ObjectType::Commit))
                .unwrap_or(false)
            {
                tags.push(raw.to_string());
            }
        }

        tags.sort();
        Ok(tags)
    }
}

pub fn latest_semver_tag(tags: &[String]) -> Option<String> {
    let mut parsed: Vec<(Version, String)> = tags
        .iter()
        .filter_map(|tag| {
            let normalized = tag.strip_prefix('v').unwrap_or(tag);
            Version::parse(normalized)
                .ok()
                .map(|version| (version, tag.clone()))
        })
        .collect();

    parsed.sort_by(|a, b| b.0.cmp(&a.0));
    parsed.first().map(|item| item.1.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_latest_semver_tag() {
        let tags = vec![
            "v1.2.0".to_string(),
            "v1.10.0".to_string(),
            "v1.3.0".to_string(),
        ];

        assert_eq!(latest_semver_tag(&tags), Some("v1.10.0".to_string()));
    }
}
