use std::env;
use std::io::{stderr, stdout};
use std::process::exit;

const MAN_PAGE: &'static str = /* @MANSTART{time} */
    r#"
NAME
    which - locate a command
SYNOPSIS
    which [ -h | --help ]
DESCRIPTION
    which prints the full path of shell commands
OPTIONS
    -h
    --help
        Print this manual page.
"#; /* @MANEND */

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stderr = stderr();

    let mut args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => {
            let _ = writeln!(stderr, "{}", "Please provide a program name");
            exit(1);
        }
        _ => match args[1].as_str() {
            "-h" | "--help" => {
                let _ = writeln!(stdout, "{}", MAN_PAGE);
                exit(0);
            }
            _ => {
                let paths = env::var("PATH").unwrap();
                args.remove(0);
                args.iter().for_each(|program| {
                    let mut exec_path = None;
                    for mut path in env::split_paths(&paths) {
                        path.push(program);
                        if path.exists() {
                            exec_path = Some(path);
                            break;
                        }
                    }

                    let _ = match exec_path {
                        Some(path) => writeln!(stdout, "{}", path.display()),
                        None => writeln!(stderr, "{} not found", program),
                    };
                });
            }
        },
    }
}
