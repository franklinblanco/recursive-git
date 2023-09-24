use clap::{Parser, Subcommand, Args};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RecursiveGitArgs {
    #[clap(subcommand)]
    pub git_operation_type: GitOperationType,
}

#[derive(Debug, Subcommand)]
pub enum GitOperationType {
    /// Pulls changes from remote
    Pull,
    /// Adds all changes to staged (git add .) then commits them all with the message inside
    Commit(CommitArgs),
    /// Pushes all committed changes
    Push,
    /// Runs git status on all
    Status,
    /// Runs git reset --hard
    HardReset,
}

#[derive(Debug, Args)]
pub struct CommitArgs {
    pub message: String
}