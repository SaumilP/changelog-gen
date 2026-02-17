#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Commit {
    pub message: String,
    pub hash: String,
}
