use colorful::{Color, Colorful};

use minigrep::{check_args_length, run_insensitive, run_sensitive, Config};

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args_length(&args).unwrap_or_else(|err| {
        let _value = format!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = Config::input_parse(&args);
    let val = run_sensitive(&input);
    let val1 = run_insensitive(&input);
    let mut sensitive_word = String::new();
    let mut insensitive_word = String::new();
    for i in &val {
        sensitive_word.push_str(i.as_str());
        sensitive_word.push('\n');
    }
    for i in &val1 {
        insensitive_word.push_str(i.as_str());
        insensitive_word.push('\n');
    }
    if sensitive_word == insensitive_word {
        let table = prettytable::table! (
            [bFY -> "RESULTS"],
            [bFC -> sensitive_word]
        );
        table.printstd();
        process::exit(4);
    }
    let table = prettytable::table!(
        [
            "CASE SENSITIVE RESULTS".gradient(Color::Green).bold(),
            "CASE INSENSITIVE RESULTS".gradient(Color::Green).bold()
        ],
        [bFC ->sensitive_word, bFC -> insensitive_word]
    );
    table.printstd();
}
