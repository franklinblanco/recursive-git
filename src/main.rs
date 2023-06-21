use std::env::current_dir;

use args::RecursiveGitArgs;
use clap::Parser;

mod args;
mod operations;



fn main() {
    let args = RecursiveGitArgs::parse();
    let current_dir = match current_dir() {
        Ok(dir) => dir,
        Err(error) => {
            println!("{error}");
            return;
        },
    };

    match args.git_operation_type {
        args::GitOperationType::Pull => {
            operations::pull(current_dir);
        },
        args::GitOperationType::Commit(commit_args) => {
            operations::commit(current_dir, commit_args.message);
        },
        args::GitOperationType::Push => {
            operations::push(current_dir);
        },
        args::GitOperationType::Status => {
            operations::status(current_dir);
        },
        
    }
}
