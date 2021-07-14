use std::env;
use std::io::{stderr, stdout, Write};
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
            let _ = stderr.write(b"Please provide an argument\n");
            exit(1);
        }
        _ => match args[1].as_str() {
            "-h" | "--help" => {
                if let Err(e) = stdout.write(MAN_PAGE.as_bytes()) {
                    let _ = stderr.write(format!("{}\n", e).as_bytes());
                    exit(1);
                };
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

                    match exec_path {
                        Some(path) => {
                            if let Err(e) = stdout.write(format!("{}\n", path.display()).as_bytes())
                            {
                                let _ = stderr.write(format!("{}\n", e).as_bytes());
                            }
                        }
                        None => {
                            let _ = stderr.write(format!("{} not found\n", program).as_bytes());
                        }
                    };
                });
            }
        },
    }
}
