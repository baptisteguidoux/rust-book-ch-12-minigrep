use std::error::Error;
use std::fs;
use std::env;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
            assert_eq!(search_case_insensitive(query, contents), vec!["Rust:", "Trust me."]);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines()
	.filter(|line| line.contains(query))
	.collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();

    contents.lines()
	.filter(|line| line.to_lowercase().contains(&query))
	.collect()
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in  results {
        println!("{}", line);
    }
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
	args.next(); // first element is the filepath

	let query = match args.next()  {
	    Some(arg) => arg,
	    None => return Err("Didn't get a query string"),
	};

	let filename = match args.next() {
	    Some(arg) => arg,
	    None => return Err("Didn't get a file name"),
	};
	
        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}
