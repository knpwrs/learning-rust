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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must provide needle and haystack arguments!")
        }
        let needle = args[1].clone();
        let haystack = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { needle, haystack, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.haystack)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.needle, &contents)
    } else {
        search_case_insensitive(&config.needle, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

// The data referenced by a slice needs to be valid for the reference to be valid
fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line.trim());
        }
    }

    results
}

fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(needle.to_lowercase().as_str()) {
            results.push(line.trim());
        }
    }

    results
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
            search(needle, haystack),
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
            search_case_insensitive(needle, haystack),
            vec!["Rust:", "Trust me."],
        );
    }
}
