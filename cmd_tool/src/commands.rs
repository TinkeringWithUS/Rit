use std::{
    fs::{self},
    io::{self, stdout, BufRead, Write},
    path::{Path, PathBuf},
    process::exit,
};

mod add;
mod commit;

use add::add_command;
use commit::commit_command;

use crate::storage::{read_metadata, search_for_metadata_folder, RitMetadata};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType {
    Init,
    Nuke,
    Add,
    Commit,
    Remove,
    Unknown,
}

pub fn execute_command(command: &CommandType, command_args: Vec<String>) -> Option<PathBuf> {
    if *command == CommandType::Init {
        return init_metadata();
    }

    let metadata_option = read_metadata();

    // special case, no metadata but calling init should suceed
    if metadata_option.is_none() {
        println!("Rit folder not found, initialize first.");
        exit(0);
    }

    let mut metadata = metadata_option.unwrap();

    println!("meta data path: {}", metadata.meta_data_folder_path);

    match command {
        CommandType::Nuke => {
            nuke_command();
            return Option::None;
        }
        CommandType::Add => {
            add_command(&mut metadata, command_args);
            return Option::None;
        }
        CommandType::Commit => {
            commit_command(); 
            return Option::None;
        }
        CommandType::Remove => {
            return Option::None;
        }
        _ => {
            return Option::None;
        }
    }
}

fn nuke_command() {
    let stdin = io::stdin();

    print!("ARE YOU SURE? Y/N: ");
    // flush to ensure print statement appears in correct order
    let _ = stdout().flush();

    let mut user_input = String::new();

    let _ = stdin.lock().read_line(&mut user_input);

    if user_input.trim().to_lowercase() == "y" {
        let metadata_folder_path = search_for_metadata_folder();

        if metadata_folder_path.is_none() {
            return;
        }

        println!("Nuking .rit");
        let _ = fs::remove_dir_all(metadata_folder_path.unwrap());
    }
}

fn init_metadata() -> Option<PathBuf> {
    let found_metadata_folder = search_for_metadata_folder();

    let mut metadata_folder_path: PathBuf = PathBuf::new();
    metadata_folder_path.push(".rit");

    if found_metadata_folder.is_none() {
        // create metadata folder
        if !create_folder(".rit") {
            println!("Error when creating metadata folder");
            return Option::None;
        }
    } else {
        metadata_folder_path = found_metadata_folder.unwrap();
        println!("Rit already initialized");
    }

    print!(
        "inside init metadata, metadata folder path: {:?}",
        &metadata_folder_path
    );

    return Option::Some(metadata_folder_path);
}

pub fn create_folder(folder_path: &str) -> bool {
    if Path::new(folder_path).exists() {
        return true;
    }

    if fs::create_dir(folder_path).is_ok() {
        return true;
    }

    return false;
}
