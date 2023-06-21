use std::{path::PathBuf, process::Command};

pub fn pull(current_dir: PathBuf) {
    let (successes, errors) = recursive_command(current_dir, &mut Command::new("git").arg("pull"));
    println!("### recursive-git pull ran {} times, successes: {successes} | errors: {errors}", successes + errors);
    println!("");
}

pub fn push(current_dir: PathBuf) {
    let (successes, errors) = recursive_command(current_dir, &mut Command::new("git").arg("push"));
    println!("### recursive-git push ran {} times, successes: {successes} | errors: {errors}", successes + errors);
    println!("");
}

pub fn commit(current_dir: PathBuf, message: String) {
    let (add_successes, add_errors) = recursive_command(current_dir.clone(), &mut Command::new("git").arg("add").arg("."));
    let (commit_successes, commit_errors) = recursive_command(current_dir, &mut Command::new("git").arg("commit").arg("-m").arg(format!("\"{message}\"")));
    println!("### recursive-git commit ran {} times, add: successes: {add_successes} | errors: {add_errors} | commit: successes: {commit_successes} | errors: {commit_errors}", add_successes + add_errors + commit_successes + commit_errors);
    println!("");
}

pub fn status(current_dir: PathBuf) {
    let (successes, errors) = recursive_command(current_dir, &mut Command::new("git").arg("status"));
    println!("### recursive-git push ran {} times, successes: {successes} | errors: {errors}", successes + errors);
    println!("");
}

fn recursive_command(path: PathBuf, command: &mut Command) -> (i32, i32) {
    let mut successes = 0;
    let mut errors = 0;
    let dir = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(error) => {
            println!("{error}");
            return (successes, errors);
        },
    };
    for subitem in dir {
        match subitem {
            Ok(entry) => {
                match entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            // Run command
                            command.current_dir(entry.path());
                            println!("{}", entry.file_name().to_string_lossy());
                            match command.output() {
                                Ok(output) => {
                                    if output.status.success() {
                                        println!("+ {}", String::from_utf8(output.stdout).expect("Stdout is not a utf-8 string."));
                                        successes += 1;
                                    } else {
                                        println!("- {}", String::from_utf8(output.stderr).expect("Stderr is not a utf-8 string."));
                                        errors += 1;
                                    }
                                },
                                Err(error) => println!("{error}"),
                            };
                        }

                    }
                    Err(error) => println!("{error}"),
                };
            },
            Err(error) => println!("{error}"),
        }
    }
    (successes, errors)
}