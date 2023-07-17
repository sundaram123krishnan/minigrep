use colored::Colorize;
use std::{fs, process};

pub fn check_args_length(arguments: &Vec<String>) -> Result<i32, String> {
    if arguments.len() == 3 {
        Ok(arguments.len() as i32)
    } else {
        Err(String::from("2 arguments are expected"))
    }
}

#[allow(dead_code)]

pub struct Config {
    pub search_string: String,
    pub filename: String,
}

impl Config {
    pub fn input_parse(args: &Vec<String>) -> Self {
        Self {
            search_string: match args.get(1) {
                Some(s) => s.to_string(),
                None => {
                    let value = format!("Cannot find the word");
                    println!("{}", value.bold().red());
                    process::exit(3);
                }
            },
            filename: match args.get(2) {
                Some(f) => f.to_string(),
                None => {
                    let value = format!("Cannot find file");
                    println!("{}", value.bold().red());
                    process::exit(4);
                }
            },
        }
    }
}

pub fn run(input: &Config) -> String {
    let contents = fs::read_to_string(&input.filename).unwrap_or_else(|err| {
        let value = format!("{}", err);
        println!("{}", value.bold().red());
        process::exit(2);
    });
    return contents;
}

pub fn word_list<'a>(words: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    for i in words.split_whitespace() {
        v.push(i);
    }
    v
}
pub fn search<'a>(word_list: &'a Vec<&str>, word: &'a String) -> Vec<&'a str> {
    let mut ok = false;
    let mut result: Vec<&str> = Vec::new();
    for i in word_list {
        if i.contains(word) {
            ok = true;
            let value = format!("{}", i);
            println!("{}", value.bold().yellow());
            result.push(i);
        }
    }
    if ok == false {
        let value = format!("Oops! No word found");
        println!("{}", value.bold().red());
    }
    result
}

#[cfg(test)]

mod tests {
    use crate::search;

    #[test]
    fn result() {
        let query = "lo".to_string();
        let contents = vec!["hello", "world", "i", "love", "this", "world"];
        let res = search(&contents, &query);
        assert_eq!(res, vec!["hello", "love"]);
    }
}
