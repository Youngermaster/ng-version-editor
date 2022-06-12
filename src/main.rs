use ng_version_editor::Config;
use regex::Regex;
use std::env;
use std::process;

// ANCHOR: here
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

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

// fn reading_file() {}
