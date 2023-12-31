use std::fs;
use std::error::Error;
use std::env;

pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config
{
    pub fn new(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3
        {
            return Err("Params Error");
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config { query: args[1].clone(), filename: args[2].clone(), case_sensitive });
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(&config.filename)?;
    if config.case_sensitive
    {
        search(&config.query, &contents);
    }
    else 
    {
        search_case_insensitive(&config.query, &contents);
    }

    for line in search(&config.query, &contents)
    {
        println!("{}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>
{
    let mut result = Vec::new();
    for line in contents.lines()
    {
        if line.contains(query)
        {
            result.push(line);
        }
    }
    return result;
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>
{
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            result.push(line);
        }
    }
    return result;
}