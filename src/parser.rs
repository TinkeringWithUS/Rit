use std::collections::HashMap;

use crate::commands::CommandType;

pub fn parse(args: &Vec<String>) -> (CommandType, Vec<String>) {
    if args.len() > 0 {
        println!("{}", args[0]);
    }

    let mut command_type = CommandType::Unknown;

    let mut command_args: Vec<String> = vec![];
    let mut command_flags: Vec<String> = vec![];

    let mut str_to_command: HashMap<String, CommandType> = HashMap::new();

    str_to_command.insert(String::from("init"), CommandType::Init);
    str_to_command.insert(String::from("nuke"), CommandType::Nuke);
    str_to_command.insert(String::from("add"), CommandType::Add);
    str_to_command.insert(String::from("commit"), CommandType::Commit);
    str_to_command.insert(String::from("rm"), CommandType::Remove);

    for (index, arg) in args.iter().enumerate() {
        let lowercase_arg = arg.trim().to_lowercase();

        println!("{} number, args: {}", index, lowercase_arg);

        // git add vs git branch origin etc...
        // for now, only support 1 keywords
        // skipping 0th index as that is "rit"
        if index == 1 {
            let possible_command = str_to_command.get(lowercase_arg.as_str());

            if possible_command.is_some() {
                command_type = *possible_command.unwrap();
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

    return (command_type, command_args);
}
