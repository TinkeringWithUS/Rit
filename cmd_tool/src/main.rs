use std::{env, time::Instant};

mod commands;
mod diff;
mod storage;
mod parser;

use commands::execute_command;
use parser::parse;

fn main() {

    let now = Instant::now(); 

    let args: Vec<String> = env::args().collect();

    if args.len() > 0 {
        println!("{}", args[0]);
    }

    let (command_type, command_args) = parse(&args);

    execute_command(&command_type, command_args);

    let elapsed = now.elapsed();

    println!("Time to Run: {:.2?}", elapsed);
}
