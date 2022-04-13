use std::env;
use std::fmt::{Error, format};
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "quick";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_query(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
        );
    }
}


// structs


pub struct GrepArgs { query: String, filename: String, case_sensitive: bool }
impl GrepArgs {
    pub fn new(args: &[String]) -> Result<GrepArgs, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let cs = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            GrepArgs {
                query: args[1].clone(),
                filename: args[2].clone(),
                case_sensitive: cs
            }
        )
    }
}

pub fn runner(grep_args: GrepArgs) -> Result<(), Box<Error>>{
    let (filename, query) = (&grep_args.filename, &grep_args.query);
    println!("Searching for {} in {}", query, filename);

    // open a file
    let mut file = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    for line in search_query(query, &contents) {
        println!("{}", line);
    }


    Ok(())
}

pub fn search_query<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut i = 1;
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
        i += 1;
    }
    results
}

pub fn search_query_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let q = query.to_lowercase();
    let mut results = Vec::new();
    let mut i = 1;

    for line in contents.lines() {
        if line.to_lowercase().contains(&q) {
            results.push(line);
        }
        i += 1;
    }
    results
}
