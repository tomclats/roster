use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("roster: usage: roster DIRECTORY");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("roster: usage: roster DIRECTORY");
        return ExitCode::from(2);
    }
    match roster::manifest(Path::new(&path)) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("roster: {err}");
            ExitCode::from(1)
        }
    }
}
