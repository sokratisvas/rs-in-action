use colored::Colorize;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn search_keyword(filename: String, keyword: String) {
    let file = File::open(filename).expect("Something went wrong reading the file");
    let bufreader = BufReader::new(file);

    for line in bufreader.lines() {
        if line.as_ref().unwrap().contains(&keyword) {
            println!(
                "{}",
                line.unwrap().replace(
                    keyword.as_str(),
                    &keyword.as_str().on_bright_blue().italic().to_string()
                )
            );
        }
    }
}

fn main() {
    let filename = String::from("sample.txt");
    let keyword = String::from("house");
    search_keyword(filename, keyword);
}
