use git2::Delta;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RepoFile {
    pub(crate) status: Delta,
    pub(crate) path: PathBuf,
}
