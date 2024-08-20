use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::panic;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }

    #[test]
    #[should_panic]
    fn fails_with_not_enough_args() {
        let args = vec!["/here".to_string(), "test".to_string()];
        if let Err(e) = Config::new(&args) {
            panic!("Application error: {}", e);
        };
    }

    #[test]
    fn passes_with_enough_args() {
        let args = [
            "/here".to_string(),
            "test".to_string(),
            "test.txt".to_string(),
        ];

        let config = Config::new(&args);

        assert_eq!(
            config.unwrap(),
            Config {
                query: "test".to_string(),
                filename: "test.txt".to_string(),
                case_sensitive: true,
            }
        )
    }

    #[test]
    fn runs_successfully() {
        let a = Config {
            query: "test".to_string(),
            filename: "poem.txt".to_string(),
            case_sensitive: true,
        };

        assert_eq!(run(a).unwrap(), ());
    }

    #[test]
    #[should_panic]
    fn fails_for_non_existent_filename() {
        let a = Config {
            query: "test".to_string(),
            filename: "I do not exist.txt".to_string(),
            case_sensitive: true,
        };

        if let Err(e) = run(a) {
            panic!("Application Error: {}", e);
        }
    }
}
