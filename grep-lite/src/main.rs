use colored::Colorize;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    keyword: String,
    #[clap(short, long)]
    filename: String
}

fn search_keyword(filename: String, keyword: String) {
    let file = File::open(filename).expect(format!("{}: {}", "fatal".red(), "something went wrong reading the file").as_str());
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
    let args = Args::parse();
    let filename = args.filename;
    let keyword = args.keyword;
    search_keyword(filename, keyword);
}
