use colored::Colorize;
use minigrep::{check_args_length, run, search, word_list, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args_length(&args).unwrap_or_else(|err| {
        let value = format!("Problem parsing arguments: {}", err);
        println!("{}", value.bold().red());
        process::exit(1);
    });

    let input = Config::input_parse(&args);
    let contents = run(&input);
    let v = word_list(&contents);
    search(&v, &input.search_string);
}
