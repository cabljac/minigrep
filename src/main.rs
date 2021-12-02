use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    vec![]
}


mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}