use std::{env, process};
use minigrep::{GrepArgs, runner};

fn main() {
    // collect the arguments and store in a vec
    let args: Vec<String> = env::args().collect();

    let grep_args = GrepArgs::new(&args)
        .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
    });

    if let Err(e) = runner(grep_args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    println!("{:?}", args);
}
