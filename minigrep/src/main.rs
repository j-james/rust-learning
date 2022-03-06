use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // & is a reference, so we don't worry abt the borrow checker until args goes out of scope
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // passing a variable without an ampersand moves it
    if let Err(e) = minigrep::run(config) {
        // this syntax is a little bit weird - you're almost implicitly running run()
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
