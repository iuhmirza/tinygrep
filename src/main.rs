use std::env;
use std::process;

use tinygrep;

fn main() -> Result<(), tinygrep::ConfigError> {
    let args = env::args();
    let config = tinygrep::Config::build(args).unwrap_or_else(|err| {
        eprintln!("failed to parse arguments: {err:?}");
        process::exit(1)
    });
    if let Err(err) = tinygrep::run(config) {
        eprintln!("failed to find, open or read file: {err:?}");
        process::exit(1)
    }
    Ok(())
}
