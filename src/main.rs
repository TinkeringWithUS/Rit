use std::collections::HashMap;
use std::env;

mod commands;
mod storage;

use commands::CommandType;

use storage::search_for_metadata_folder;
use storage::create_metadata_folder;

fn main() {

    let found_metadata_folder = search_for_metadata_folder(".rit");

    if found_metadata_folder.is_none() {
        create_metadata_folder(".rit");
    }

    let args: Vec<_> = env::args().collect();

    if args.len() > 0 {
        println!("{}", args[0]);
    }

    let mut command_type = CommandType::Unknown;

    let mut command_args: Vec<String> = vec![];
    let mut command_flags: Vec<String> = vec![];

    let mut str_to_command: HashMap<String, CommandType> = HashMap::new();

    str_to_command.insert(String::from("add"), CommandType::Add);
    str_to_command.insert(String::from("commit"), CommandType::Commit);
    str_to_command.insert(String::from("rm"), CommandType::Remove);

    for (index, arg) in args.iter().enumerate() {
        let lowercase_arg = arg.trim().to_lowercase();

        println!("{} number, args: {}", index, lowercase_arg);

        // git add vs git branch origin etc...
        // for now, only support 1 keywords
        if index == 1 {
            if str_to_command.contains_key(lowercase_arg.as_str()) {
                command_type = *str_to_command.get(lowercase_arg.as_str()).unwrap();
            }
        } else if lowercase_arg.len() > 1 && lowercase_arg.chars().nth(0).unwrap() == '-' {
            command_flags.push(lowercase_arg);
        } else {
            // push onto command args
            command_args.push(lowercase_arg);
        }
    }

    println!("command type: {:?}", command_type);

    for (_, command_arg_type) in command_args.iter().enumerate() {
        println!("Argument: {}", command_arg_type);
    }

    for (_, command_flag) in command_flags.iter().enumerate() {
        println!("Flag: {}", command_flag);
    }
}
