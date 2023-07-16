use std::{env, fs, process};

fn check_args_length(arguments: &Vec<String>) -> Result<i32, String> {
    if arguments.len() == 3 {
        Ok(arguments.len() as i32)
    } else {
        Err(String::from("2 arguments are expected"))
    }
}

struct Config {
    search_string: String,
    filename: String,
}

impl Config {
    fn input_parse(args: &Vec<String>) -> Self {
        Self {
            search_string: match args.get(1) {
                Some(s) => s.to_string(),
                None => {
                    println!("Cannot find the word");
                    process::exit(3);
                }
            },
            filename: match args.get(2) {
                Some(f) => f.to_string(),
                None => {
                    println!("Cannot find file");
                    process::exit(4);
                }
            },
        }
    }
}

fn run(input: &Config) -> String {
    let contents = fs::read_to_string(&input.filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(2);
    });
    return contents;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args_length(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = Config::input_parse(&args);
    run(&input);
}
