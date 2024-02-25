use std::{env, process};
use minigrep::SearchInfo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search = SearchInfo::build(&args).unwrap_or_else(|error| {
        println!("Error : {error}");
        process::exit(1);
    });

    minigrep::run(search);

    // let res = search_str_in_file(arg1, arg2);
    // println!("Search res: {:?}", res.expect("Sorry search failed!"));
}

