use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);
    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not nough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    mod config {
        use super::super::*;
        use core::panic;
        #[test]
        #[should_panic]
        pub(crate) fn fails_with_not_enough_args() {
            let args = ["/here".to_string(), "test".to_string()];
            if let Err(e) = Config::new(&args) {
                panic!("Application error: {}", e);
            };
        }

        #[test]
        pub(crate) fn passes_with_enough_args() {
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
                    filename: "test.txt".to_string()
                }
            )
        }
    }

    mod run {
        use core::panic;

        use super::super::*;

        #[test]
        fn runs_successfully() {
            let a = Config {
                query: "test".to_string(),
                filename: "poem.txt".to_string(),
            };

            assert_eq!(run(a).unwrap(), ());
        }

        #[test]
        #[should_panic]
        fn fails_for_non_existent_filename() {
            let a = Config {
                query: "test".to_string(),
                filename: "I do not exist.txt".to_string(),
            };

            if let Err(e) = run(a) {
                panic!("Application Error: {}", e);
            }
        }
    }
}
