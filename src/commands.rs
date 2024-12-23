// use std::{fs::read_dir, path::PathBuf};

// mod storage;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType {
    Add,
    Commit,
    Remove,
    Unknown,
}

// pub fn execute_command(command: &CommandType, command_args: Vec<String>) {
//     match command {
//         CommandType::Add => {
//             add(command_args);
//         }
//         CommandType::Commit => {}
//         CommandType::Remove => {}
//         _ => println!(),
//     }
// }

// fn add(command_args: Vec<String>) {
//     let mut files_to_add: Vec<PathBuf> = Vec::new();

//     // for now, only handle .
//     for (_, command_arg) in command_args.iter().enumerate() {
//         if command_arg == "." {
//             let entries = read_dir(command_arg).unwrap();

//             for entry in entries {
//                 // entry?.path().is_dir();
//                 let entry_path = entry.unwrap().path();

//                 add_files(entry_path, &mut files_to_add);
//             }
//         }
//     } 

//     for path in files_to_add {
//         println!("added file: {:?}", path);
//     }
// }

// fn add_files(path: PathBuf, added_files: &mut Vec<PathBuf>) {
//     if path.is_dir() {
//         let subdir_entries = read_dir(path).unwrap();

//         for dir_entry in subdir_entries { 
//             let dir_entry_path = dir_entry.unwrap().path();

//             add_files(dir_entry_path, added_files);
//         }
//     } 
//     else if path.is_file() {
//         added_files.push(path);
//     }
// }
