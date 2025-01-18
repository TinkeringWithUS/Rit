use std::fs::{self, File};
use std::path::Path;
// for now, use a csv format
use std::path::PathBuf;
use std::time::Instant;
use std::{io, vec};

use std::collections::HashMap;

use std::io::{BufRead, Read};

mod path_to_hash;

// exposes storage utils for rest of project to use
pub mod utils;

use path_to_hash::init_path_to_obj_hash_file;
use path_to_hash::PathToHash;
use utils::{create_file, create_folder, hash_file, recursive_dir_search, search_dir, zip_file};

// structure
// 1. read everything inside dir
// 2. compare against file objs inside Metadata
//      1. same file, different path or name, (use file_hash)
//      2. changed file, same path (use MetadataEntry file_path)
//      3. both (can treat either as 1. deleted file + new file 2. (1 + 2))
// 3. uncompress file obj and compare against current file (for 2nd case)
//    Meyer's diffing algorithm
// 4. diffed lines will consist of line + filename or path + differing lines (Diff)
// 5. store Diffs in a commit
// 6. save commit and update commit log (+ current commit)
// 7. save all of this inside the metadata folder

enum ChangeType {
    ADDITION,
    SUBTRACTION,
    UNCHANGED,
}

pub struct RitMetadata {
    pub hash_to_file_obj_path: HashMap<String, String>,
    pub meta_data_folder_path: String,
    pub current_commit_id: String,
    pub commit_log: Vec<CommitLogEntry>,
    pub path_to_hash_objs: PathToHash,
    pub filename_patterns_to_ignore: Vec<String>,
}

struct CommitLogEntry {
    hash: String,
    message: String,
}

// struct Commit {
//     hash: String,
//     diff_list: Vec<FileDifference>,
// }

// struct FileDifference {
//     file_hash: String,           // hash of previously commited file object
//     original_file_path: String,  // original relative file path
//     changed_file_path: String,   // used if file was moved or renamed
//     obj_path: String,            // zipped, committed file object path
//     differences: Vec<DiffRange>, // stores ranges of changes
// }

// struct DiffRange {
//     starting_line: u32, // inclusive
//     ending_line: u32,   // inclusive
//     changed_lines: Vec<String>,
//     change_type: ChangeType,
// }

pub fn read_metadata() -> Option<RitMetadata> {
    let found_metadata_folder = search_for_metadata_folder();

    let metadata_folder_path: PathBuf;

    if found_metadata_folder.is_none() {
        // can't find it, up to user to call init
        return Option::None;
    } else {
        metadata_folder_path = found_metadata_folder.unwrap();
    }

    const COMMIT_LOG_FILENAME: &str = "commit_log.txt";
    // const COMMIT_STORE_FILENAME: &str = "commits.txt";

    let metadata_folder_path_str = metadata_folder_path.to_str().unwrap();

    let commit_log_option = init_commit_log(COMMIT_LOG_FILENAME, metadata_folder_path_str);

    if commit_log_option.is_none() {
        return Option::None;
    }

    let commit_log = commit_log_option.unwrap();

    // for commit in commit_log.iter() {
    //     let commit_hash: &str = &commit.hash;
    //     let found_path = search_dir(metadata_folder_path_str, commit_hash, false);

    //     // find the zipped file object with commit hash
    // }

    let path_to_hash_objs = init_path_to_obj_hash_file(metadata_folder_path.to_str().unwrap());

    if path_to_hash_objs.is_none() {
        return Option::None;
    }

    let metadata = RitMetadata {
        hash_to_file_obj_path: HashMap::new(),
        meta_data_folder_path: metadata_folder_path_str.to_string(),
        current_commit_id: "0".to_string(),
        commit_log,
        path_to_hash_objs: path_to_hash_objs.unwrap(),
        filename_patterns_to_ignore: read_ignore_file(Path::new(metadata_folder_path_str)),
    };

    return Option::Some(metadata);
}

fn read_ignore_file(metadata_folder_path: &Path) -> Vec<String> {
    let mut dir_to_add_buf = metadata_folder_path.parent().unwrap().to_path_buf();

    dir_to_add_buf.push(".ritignore");

    let file_contents = fs::read_to_string(dir_to_add_buf);

    // can silently ignore error
    if !file_contents.is_err() {
        return file_contents
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();
    }

    return vec![];
}

// metadata_folder_path should be a relative path
pub fn record_added_files(
    metadata: &mut RitMetadata,
    metadata_folder_path: &str,
    file_paths_to_add: &Vec<PathBuf>,
) -> bool {
    let metadata_folder_path_buf = PathBuf::new();
    if !metadata_folder_path_buf.is_relative() {
        println!("RECORD ADDED FILES REQUIRES A RELATIVE METADATA FOLDER PATH");
        return false;
    }

    let file_obj_path = format!("{}/objs/", metadata_folder_path);
    if !create_folder(&file_obj_path) {
        println!(
            "Creating obj folder failed in record added files obj path: {}",
            &file_obj_path
        );
        return false;
    }

    let now = Instant::now();
    
    for file_path_to_add in file_paths_to_add {
        println!("file path in record added files: {:?}", file_path_to_add);

        if file_path_to_add.file_name().is_none() {
            println!("File name is none in record added files");
            return false;
        }

        let filename_to_add = file_path_to_add.file_name().unwrap().to_str().unwrap();

        let found_file = recursive_dir_search(&file_obj_path, filename_to_add, false);

        // adds new files / changed files / or moved files
        // TODO: update path to hash log.txt when files moves, since we
        // don't modify it
        if found_file.is_none() {
            // create the zipped file and zip
            let zipped_file_hash_str = create_zip_archive(
                &file_path_to_add.to_str().unwrap().to_string(),
                &file_obj_path,
                metadata_folder_path,
            );

            if zipped_file_hash_str.is_none() {
                return false;
            }

            metadata.path_to_hash_objs.add_new_entry(
                &zipped_file_hash_str.unwrap(),
                file_path_to_add.to_str().unwrap(),
            );
        }
    }

    metadata
        .path_to_hash_objs
        .record_path_to_hashes(&metadata.meta_data_folder_path);

    let elapsed = now.elapsed();

    println!("record add files, elapsed time: {:?}", elapsed);

    return true;
}

fn create_zip_archive(
    file_path_to_add: &String,
    file_obj_path: &String,
    metadata_folder_path: &str,
) -> Option<String> {
    let zipped_file_hash = hash_file(file_path_to_add);

    if zipped_file_hash.is_none() {
        println!("zipped_file_hash is none");
        return Option::None;
    }

    let zipped_file_hash_str = zipped_file_hash.unwrap();

    let new_zipped_file = create_file(&zipped_file_hash_str, &file_obj_path);

    if new_zipped_file.is_none() {
        println!("new_zipped_file is none");
        return Option::None;
    }

    let zipped_filepath_str = format!("{}/objs/{}", metadata_folder_path, zipped_file_hash_str);

    let zipped_filepath = Path::new(&zipped_filepath_str);

    if !zip_file(&zipped_filepath, file_path_to_add) {
        println!("!zip_file");
        return Option::None;
    }

    println!("record new entry, file path: {}", file_path_to_add);

    return Option::Some(zipped_file_hash_str);
}

fn read_file(filepath_to_read: &str, buffer: &mut Vec<u8>) -> bool {
    let original_file = File::open(filepath_to_read);

    if original_file.is_err() {
        println!("File name is none in record added files");
        return false;
    }

    if original_file.unwrap().read_to_end(buffer).is_err() {
        println!(
            "Failed to read original file, filename: {:?}",
            filepath_to_read
        );
        return false;
    }

    return true;
}

fn init_commit_log(
    commit_log_filename: &str,
    metadata_folder_path_str: &str,
) -> Option<Vec<CommitLogEntry>> {
    let mut commit_log_option = read_commit_log(metadata_folder_path_str, commit_log_filename);

    if commit_log_option.is_none() {
        let created_commit_log = create_file(commit_log_filename, metadata_folder_path_str);

        if created_commit_log.is_none() {
            return Option::None;
        }

        commit_log_option = Option::Some(vec![]);
    }

    return commit_log_option;
}

fn read_commit_log(
    metadata_folder_path: &str,
    commit_filename: &str,
) -> Option<Vec<CommitLogEntry>> {
    let commit_log_path = format!("{}/{}", metadata_folder_path, commit_filename);

    let file = File::open(commit_log_path);

    if file.is_err() {
        return Option::None;
    }

    let lines_in_file = io::BufReader::new(file.unwrap()).lines();

    let mut commit_log: Vec<CommitLogEntry> = Vec::new();

    for line in lines_in_file.flatten() {
        let tokens = line.split(" ");

        let mut commit_entry = CommitLogEntry {
            hash: "".to_string(),
            message: "".to_string(),
        };

        for (index, token) in tokens.enumerate() {
            match index {
                0 => commit_entry.hash = token.to_string(),
                1 => commit_entry.message = token.to_string(),
                _ => {}
            }
        }

        commit_log.push(commit_entry);
    }

    return Option::Some(commit_log);
}

pub fn search_for_metadata_folder() -> Option<PathBuf> {
    let metadata_folder_name = ".rit";

    // search in current directory
    let mut current_path = ".".to_string();

    for _ in 1..10 {
        let found_dir = search_dir(&current_path, metadata_folder_name, true);

        if found_dir.is_some() {
            let copy = found_dir.unwrap();

            return Option::Some(copy);
        }

        if found_dir.is_none() {
            let up_directory_path = "../";

            current_path = format!("{}{}", up_directory_path, current_path);

            // println!("Current path to search: {}", current_path);

            let mut buf = PathBuf::new();

            buf.push(&current_path);

            if !buf.exists() {
                return Option::None;
            }
        }
    }

    return Option::None;
}
