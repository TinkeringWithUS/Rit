

use std::env;

pub fn parse(cmd_args: Vec<String>) {
    if cmd_args.len() > 0 {
        println!("{}", args[0]);
    }

    let mut command = CommandType::Unknown;

    let mut command_args: Vec<String> = vec![];

    for (index, arg) in args.iter().enumerate() {
        let lowercase_arg = arg.trim().to_lowercase();

        println!("{} number, args: {}", index, lowercase_arg);

        // git add vs git branch origin etc...
        // for now, only support 1 keywords
        if index == 1 {
            match lowercase_arg {
                _ if lowercase_arg == "add" => {
                    println!("inside of match case for add");
                    command = CommandType::Add;
                }
                _ if lowercase_arg == "rm" => {
                    command = CommandType::Remove;
                }
                _ if lowercase_arg == "commit" => {
                    command = CommandType::Commit;
                }
                _ => println!("not matched, lowercase arg: {}", lowercase_arg),
            }
        } else {
            command_args.push(lowercase_arg);
        }
    }
}