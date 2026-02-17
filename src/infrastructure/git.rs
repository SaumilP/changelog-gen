use git2::Repository;
use anyhow::Result;
use crate::traits::git::GitRepository;
use crate::domain::commit::Commit;

pub struct Git2Repository;

#[async_trait::async_trait]
impl GitRepository for Git2Repository {
    async fn get_commits(&self) -> Result<Vec<Commit>> {
        let repo = Repository::open(".")?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = vec![];

        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            commits.push(Commit {
                message: commit.message().unwrap_or("").into(),
                hash: oid.to_string(),
            });
        }

        Ok(commits)
    }
}
