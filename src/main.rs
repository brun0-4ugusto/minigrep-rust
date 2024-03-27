use std::{env, process};
use minigrep::Config;

fn main() {
    // Vetores Vec<T> são uma espécie de lista que coloca os valores
    // um ao lado do outro na memória, guardam valores do mesmo tipo.

    let args: Vec<String> = env::args().collect();
    let args = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem: {e}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    };
}





