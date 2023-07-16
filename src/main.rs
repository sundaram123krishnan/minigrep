use std::env;
fn check_args_length(arguments: &Vec<String>) -> Result<usize, String> {
    if arguments.len() > 1 && arguments.len() < 4 {
        Ok(arguments.len())
    } else {
        Err(String::from("Enter 2 arguments"))
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let sz = check_args_length(&args);
    let _sz = match sz {
        Ok(n) => n,
        Err(_) => panic!("Less arguments provided"),
    };

    let _search_string = args.get(1);
    let _filename = args.get(2);

    let _search_string = match _search_string {
        Some(s) => s,
        None => panic!("Pls enter string to search"),
    };
    let _filename = match _filename {
        Some(f) => f,
        None => panic!("Pls enter valid filename"),
    };
}
