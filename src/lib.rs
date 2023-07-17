use colorful::Colorful;
use std::{fs, process};

pub fn check_args_length(arguments: &Vec<String>) -> Result<i32, String> {
    if arguments.len() == 3 {
        Ok(arguments.len() as i32)
    } else {
        Err(String::from("2 arguments are expected"))
    }
}

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

pub fn run_insensitive(input: &Config) -> Vec<String> {
    let contents = fs::read_to_string(&input.filename).unwrap();
    let mut result = Vec::new();
    for line in case_insensitive_search(&contents, &input.search_string) {
        let value = format!("{}", line);
        result.push(value);
    }
    return result;
}

pub fn run_sensitive(input: &Config) -> Vec<String> {
    let contents = fs::read_to_string(&input.filename).unwrap();
    let mut result = Vec::new();
    for line in search(&contents, &input.search_string) {
        let value = format!("{}", line);
        result.push(value);
    }
    return result;
}
pub fn word_list<'a>(words: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    for i in words.split_whitespace() {
        v.push(i);
    }
    v
}

pub fn search<'a>(word_list: &'a str, word: &'a String) -> Vec<&'a str> {
    let mut ok: bool = false;
    let mut result = Vec::new();
    for i in word_list.lines() {
        if i.contains(word) {
            ok = true;
            result.push(i);
        }
    }
    if ok == false {
        let value = format!("Oops! No word found");
        println!("{}", value.bold().red());
    }
    result
}
pub fn case_insensitive_search<'a>(word_list: &'a str, word: &'a String) -> Vec<&'a str> {
    let mut ok: bool = false;
    let mut result = Vec::new();
    let temp_word = word.to_lowercase();

    for i in word_list.lines() {
        let temp = i.to_lowercase();
        if temp.contains(&temp_word) {
            ok = true;
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
        let content = "Hello how are you i hope you are good";
        let res = search(&content, &query);
        assert_eq!(res, vec!["Hello how are you i hope you are good"]);
    }
}
