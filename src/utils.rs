use crate::models::RepoFile;
use git2::Delta;

pub fn sort_repo_files_by_status(mut files: Vec<RepoFile>) -> Vec<RepoFile> {
    files.sort_by(|a, b| {
        let status_to_ord = |status: &Delta| match status {
            Delta::Unmodified => 0,
            Delta::Ignored => 1,
            Delta::Untracked => 2,
            Delta::Added => 3,
            Delta::Deleted => 4,
            Delta::Modified => 5,
            Delta::Renamed => 6,
            Delta::Copied => 7,
            Delta::Typechange => 8,
            Delta::Unreadable => 9,
            Delta::Conflicted => 10,
        };
        status_to_ord(&a.status).cmp(&status_to_ord(&b.status))
    });

    files
}
