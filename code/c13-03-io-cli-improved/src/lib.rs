use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;

pub struct Config {
    needle: String,
    haystack: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip executable name
        let needle = match args.next() {
            Some(s) => s,
            None => return Err("No needle given"),
        };
        let haystack = match args.next() {
            Some(s) => s,
            None => return Err("No haystack given"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { needle, haystack, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.haystack)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for result in search(&config.needle, &contents, config.case_sensitive) {
        println!("{}", result);
    }

    Ok(())
}

// The data referenced by a slice needs to be valid for the reference to be valid
fn search<'a>(needle: &str, haystack: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let iter = haystack
        .lines()
        .map(|line| line.trim());
    if case_sensitive {
        iter.filter(|line| line.contains(needle))
            .collect()
    } else {
        iter.filter(|line| line.to_lowercase().contains(needle.to_lowercase().as_str()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let needle = "duct";
        let haystack = "
          Rust:
          safe, fast, productive.
          Pick three.
          Duct tape.
        ";
        assert_eq!(
            search(needle, haystack, true),
            vec!["safe, fast, productive."],
        );
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = "
          Rust:
          safe, fast, productive.
          Pick three.
          Trust me.
        ";
        assert_eq!(
            search(needle, haystack, false),
            vec!["Rust:", "Trust me."],
        );
    }
}
