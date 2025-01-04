use std::{
    fs::{self, read_dir},
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::exit,
    vec,
};

use crate::storage::{
    add_files, read_metadata, record_added_files, search_for_metadata_folder, RitMetadata,
};

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

    println!("ARE YOU SURE? Y/N: ");

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

fn add_command(metadata: &mut RitMetadata, command_args: Vec<String>) {
    let directories_to_ignore = vec!["target", ".vscode"];

    let metadata_folder_path = search_for_metadata_folder();

    if metadata_folder_path.is_none() {
        return;
    }

    let mut files_to_add: Vec<PathBuf> = Vec::new();

    // for now, only handle .
    for (_, command_arg) in command_args.iter().enumerate() {
        if command_arg == "." {
            let entries = read_dir(command_arg).unwrap();

            for entry in entries {
                // entry?.path().is_dir();
                let entry_path = entry.unwrap().path();

                if !directories_to_ignore
                    .contains(&entry_path.file_name().unwrap().to_str().unwrap())
                {
                    add_files(entry_path, &mut files_to_add);
                }
            }
        }
    }

    for path in &files_to_add {
        println!("added file: {:?}", path);
    }

    let binding = metadata_folder_path.unwrap();
    let metadata_path_str = binding.to_str().unwrap();

    record_added_files(metadata, metadata_path_str, &files_to_add);
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
