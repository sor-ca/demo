use std::error::Error;
use std::fs;
use std::env;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len()<3 {
            return Err("Not enough arguments");
        };   
        let query = args[1].clone();
        let filename = args[2].clone();
        //let case_sensitive =
            //if args[3]=="insensitive"{false
            //} else { true};

        let case_sensitive=env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive,})
    }
}
//#[derive(Debug)]
pub fn run(config:Config)->Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let result = 
        if config.case_sensitive {
            search(&config.query, &content)
        } else {
            search_insensitive(&config.query, &content)
        };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
            let query="duct";
            let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct here:)";
            assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn case_insensitive() {
            let query="Rust";
            let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
            assert_eq!(vec!["Rust:","Trust me"], search_insensitive(query, content));
    }

}
pub fn search<'a>(query:&str, content:&'a str)-> Vec<&'a str> {
    let mut result=Vec::new();
    for line in content.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
pub fn search_insensitive<'a>(query:&str, content:&'a str)-> Vec<&'a str> {
    let mut result=Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()){
            result.push(line);
        }
    }
    result
}

