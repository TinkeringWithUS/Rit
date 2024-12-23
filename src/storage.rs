// for now, use a csv format
use std::fs;
use std::{fs::read_dir, path::PathBuf};


pub fn create_metadata_folder(metadata_path: &str) -> bool {
    if fs::create_dir(metadata_path).is_ok() {
        return true;
    }

    return false;
}

pub fn search_for_metadata_folder(metadata_folder_name: &str) -> Option<PathBuf> {
    // search in current directory

    let mut current_path = ".".to_string();

    for _ in 1..10 {
        let found_dir = search_dir(&current_path, metadata_folder_name, true);

        if !found_dir.is_none() {
            let copy = found_dir.unwrap();

            println!("Found directory: {:?}", copy);

            // let copy = found_dir.clone().unwrap();

            return Option::Some(copy);
        }

        if found_dir.is_none() {
            let up_directory_path = "../";

            current_path = format!("{}{}", up_directory_path, current_path);

            println!("Current path to search: {}", current_path);

            let mut buf = PathBuf::new();

            buf.push(&current_path);

            if !buf.exists() {
                return Option::None;
            }
        }
    }

    return Option::None;
}

// Pre condition: dir_path_to_search must exists
fn search_dir(dir_path_to_search: &str, entry_name_to_find: &str, is_dir: bool) -> Option<PathBuf> {
    let current_dir_entries = read_dir(dir_path_to_search).unwrap();

    for dir_entry in current_dir_entries {
        let dir_entry_path = dir_entry.unwrap().path();

        let dir_entry_name = dir_entry_path.file_name().unwrap().to_str().unwrap();

        let current_dir_entry_is_dir = is_dir && dir_entry_path.is_dir();
        let current_dir_entry_is_file = !is_dir && dir_entry_path.is_file();

        if (current_dir_entry_is_dir || current_dir_entry_is_file)
            && dir_entry_name.contains(entry_name_to_find)
        {
            return Option::Some(dir_entry_path);
        }
    }

    return Option::None;
}

