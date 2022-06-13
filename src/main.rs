use ng_version_editor::Config;
use regex::Regex;
use std::env;
use std::process;

use std::io::{BufRead, BufReader};
use std::{fs, fs::OpenOptions};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = ng_version_editor::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
    edit_version();
    remove_tag();
    skip_test();
}

fn skip_test() {
    println!("Skip TEST");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("file.txt")
        .expect("file.txt doesn't exist or so");

    let mut lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    lines.remove(1);

    fs::write("file.txt", lines).expect("Can't write");
}

fn edit_version() {
    let parenthesis_regex = Regex::new(r#""(.*?)""#).unwrap();
    let text = r#"dsaidajsdio "test 0", "test 1" stuf stuff"#;
    for capture in parenthesis_regex.captures_iter(text) {
        println!("Quoted text => {}", &capture[0]);
    }
}

fn remove_tag() {
    println!("Removing tag...");
}
