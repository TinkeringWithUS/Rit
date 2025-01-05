use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::storage::{
    record_added_files, search_for_metadata_folder, utils::hash_file, RitMetadata,
};

pub fn add_command(metadata: &mut RitMetadata, command_args: Vec<String>) {
    let patterns_to_ignore = &metadata.filename_patterns_to_ignore;

    let metadata_folder_path = search_for_metadata_folder();

    if metadata_folder_path.is_none() {
        return;
    }

    let mut files_to_add: Vec<PathBuf> = Vec::new();

    // for now, only handle .
    for (_, command_arg) in command_args.iter().enumerate() {
        let starting_add_path = command_arg.clone();

        add_files(
            &metadata,
            Path::new(&starting_add_path).to_path_buf(),
            &mut files_to_add,
            patterns_to_ignore,
        );
    }

    for path in &files_to_add {
        println!("added file: {:?}", path);
    }

    if files_to_add.len() == 0 {
        println!("Nothing to add");
        return;
    }

    let binding = metadata_folder_path.unwrap();
    let metadata_path_str = binding.to_str().unwrap();

    record_added_files(metadata, metadata_path_str, &files_to_add);
}

fn add_files(
    metadata: &RitMetadata,
    path: PathBuf,
    added_files: &mut Vec<PathBuf>,
    patterns_to_ignore: &Vec<String>,
) {
    let should_be_ignored = patterns_to_ignore
        .iter()
        .any(|pattern| pattern_is_match(pattern, path.to_str().unwrap().to_string().as_str()));

    // println!("add files, path: {:?}", path);

    if !should_be_ignored {
        if path.is_dir() {
            let subdir_entries = read_dir(path).unwrap();

            for dir_entry in subdir_entries {
                let dir_entry_path = dir_entry.unwrap().path();

                add_files(metadata, dir_entry_path, added_files, patterns_to_ignore);
            }
        } else if path.is_file() {
            only_add_changed_file(metadata, path.as_path(), added_files);
        }
    }
}

fn only_add_changed_file(metadata: &RitMetadata, path: &Path, added_files: &mut Vec<PathBuf>) {
    let path_str = path.to_str().unwrap();

    if path.is_file() {
        let file_hash = hash_file(path.to_str().unwrap()).unwrap();

        let previous_relative_path_option =
            metadata.path_to_hash_objs.get_relative_filepath(&file_hash);
        let previous_file_hash_option =
            metadata.path_to_hash_objs.get_hash(&path.to_str().unwrap());

        if previous_relative_path_option.is_some() {
            let previous_relative_path = previous_relative_path_option.unwrap();
            // file has been moved, so add
            if previous_relative_path != path_str {
                added_files.push(path.to_path_buf());
            }
        } else if previous_file_hash_option.is_some() {
            let previous_file_hash = previous_file_hash_option.unwrap();
            // file has changed, so add
            if previous_file_hash != file_hash {
                added_files.push(path.to_path_buf());
            }
        }
        // this file isn't recorded
        else {
            added_files.push(path.to_path_buf());
        }
    }
}

fn pattern_is_match(pattern: &str, text_to_match: &str) -> bool {
    return text_to_match.contains(pattern);
}
