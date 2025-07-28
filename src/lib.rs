use std::env;
use std::fs;
use std::io;

pub struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingQuery,
    MissingFilePath,
    InvalidCase,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, ConfigError> {
        args.next();

        let query = args.next().ok_or(ConfigError::MissingQuery)?;
        let file_path = args.next().ok_or(ConfigError::MissingFilePath)?;
        let case_sensitive = match args.next() {
            Some(str) => match str.as_str() {
                "sen" => Ok(true),
                "ins" => Ok(false),
                _ => Err(ConfigError::InvalidCase),
            },
            None => Ok(true),
        }?;

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), io::Error> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }
    Ok(())
}

fn search<'a>(target: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(target)).collect()
}

fn search_case_insensitive<'a>(target: &str, text: &'a str) -> Vec<&'a str> {
    let target = target.to_lowercase();
    text.lines()
        .filter(|line| line.to_lowercase().contains(&target))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "aft";
        let contents = "\
There are times that walk from you,
like some passing afternoon.
Summer warms the open window...";
        assert_eq!(
            vec!["like some passing afternoon."],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "The";
        let contents = "\
There are times that walk from you,
like some passing afternoon.
Summer warms the open window...";
        assert_eq!(
            vec!["There are times that walk from you,"],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "tHe";
        let contents = "\
There are times that walk from you,
like some passing afternoon.
Summer warms the open window...";
        assert_eq!(
            vec![
                "There are times that walk from you,",
                "Summer warms the open window..."
            ],
            search_case_insensitive(query, contents)
        )
    }
}
