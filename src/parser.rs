use regex::Regex;
use std::collections::BTreeMap;
use anyhow::Result;
use crate::domain::commit::Commit;

pub type GroupedCommits = BTreeMap<String, Vec<String>>;

pub fn group_commits(
    commits: Vec<Commit>,
    conventional: bool,
) -> Result<GroupedCommits> {

    let mut groups : GroupedCommits = BTreeMap::new();

    if conventional {
        
        let re = Regex::new(r"^(feat|fix|docs|chore|refactor|test)(\(.+\))?: (.+)")?;

        for commit in commits {
            if let Some(caps) = re.captures(&commit.message) {
                let kind = caps.get(1).unwrap().as_str();
                let desc = caps.get(3).unwrap().as_str();
                groups.entry(kind.to_string())
                .or_default()
                .push(desc.to_string());
            }
        }

    } else {
        groups.insert("changes".into(),
    commits.into_iter().map(|c| c.message).collect());
    }
    
    Ok(groups)
}