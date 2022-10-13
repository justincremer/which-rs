use std::{
    env,
    io::{self, Stderr, Stdout, Write},
    process,
};

const MAN_PAGE: &str = /* @MANSTART{time} */
    r#"
NAME
    which - locate a command
SYNOPSIS
    which [ -h | --help | help ]
DESCRIPTION
    prints the full path of commands
OPTIONS
    -h Print this manual page.
"#; /* @MANEND */

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let stdout = io::stdout();
    let stderr = io::stderr();

    if args.len() < 2 {
        stderr.lock().write_all(b"Please provide a name\n")?;
        process::exit(1);
    }

    let mut command = Command::new(args, Some(stdout), Some(stderr));
    command.execute()
}

struct Command {
    args: Vec<String>,
    stdout: Stdout,
    stderr: Stderr,
}

impl Command {
    pub fn new(args: Vec<String>, stdout: Option<Stdout>, stderr: Option<Stderr>) -> Self {
        Self {
            args,
            stdout: stdout.unwrap_or_else(io::stdout),
            stderr: stderr.unwrap_or_else(io::stderr),
        }
    }

    pub fn execute(&mut self) -> io::Result<()> {
        match self.args[1].as_str() {
            "-h" | "--help" | "help" => self.help(),
            _ => self.run(),
        }
    }

    fn help(&mut self) -> io::Result<()> {
        map_io_result!(self.write_out(MAN_PAGE))
    }

    fn run(&mut self) -> io::Result<()> {
        let paths = env::var("PATH").unwrap();
        self.args.remove(0);
        self.args
            .to_owned()
            .iter()
            .try_for_each(|program| -> io::Result<()> {
                let mut exec_path = None;
                for mut path in env::split_paths(&paths) {
                    path.push(program);
                    if path.exists() {
                        exec_path = Some(path);
                        break;
                    }
                }

                match exec_path {
                    Some(path) => self.write_out(format!("{}\n", path.display()).as_str()),
                    None => self.write_err(format!("{} not found\n", program).as_str()),
                }
            })?;

        Ok(())
    }

    fn write_out(&mut self, message: &str) -> io::Result<()> {
        map_io_result!(self.stdout.lock().write(message.as_bytes()))
    }

    fn write_err(&mut self, message: &str) -> io::Result<()> {
        map_io_result!(self.stderr.lock().write(message.as_bytes()))
    }
}

#[macro_export]
macro_rules! map_io_result {
    ($result:expr) => {
        match $result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    };
}
