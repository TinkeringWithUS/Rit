// use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
// use std::path::Path;
// use std::r::{Enumerate, Map};
// for now, use a csv format
use std::{fs, vec};

// use std::io::{BufRead, Write};

pub struct PathToHash {
    entries: Vec<PathToHashEntry>,
}
pub struct PathToHashEntry {
    hash: String,
    relative_filepath: String,
}

impl PathToHash {
    pub fn record_path_to_hashes(&self, metadata_folder_path: &str) {
        for entry in &self.entries {
            entry.record_hash_entry(metadata_folder_path);
        }
    }

    // use hashmap no?
    pub fn record_new_entries(&mut self, new_entries: Vec<PathToHashEntry>) {
        for new_entry in &new_entries {
            self.record_new_entry(&new_entry.hash, &new_entry.relative_filepath);
        }
    }

    pub fn record_new_entry(&mut self, file_hash: &str, relative_filepath: &str) {
        self.entries.push(PathToHashEntry {
            hash: file_hash.to_string().clone().to_string(),
            relative_filepath: relative_filepath.to_string().clone().to_string()
        });
    }

    // str is more or less std::string_view in c++ (tldr; read only string)
    pub fn get_hash(&self, relative_filepath: &str) -> Option<&str> {
        for entry in &self.entries {
            if entry.relative_filepath == relative_filepath {
                // return Option::Some(&entry.hash);
                return Option::Some(entry.hash.as_str());
            }
        }
        return Option::None;
    }

    pub fn get_relative_filepath(&mut self, file_hash: &str) -> Option<&str> {
        for entry in &self.entries {
            if entry.hash == file_hash {
                // return Option::Some(&entry.hash);
                return Option::Some(&entry.relative_filepath.as_str());
            }
        }
        return Option::None;
    }
}

impl PathToHashEntry {
    pub fn record_hash_entry(&self, metadata_folder_path: &str) {
        let log_path = format!("{}/path_to_hash.txt", metadata_folder_path);

        let path_to_hashes_log  = OpenOptions::new().append(true).open(log_path);

        if path_to_hashes_log.is_err() {
            println!("Failed to open path to hashes log: {:?}", path_to_hashes_log);
            return;
        }

        let content = format!("{} {}\n", self.hash, self.relative_filepath);

        println!("content: {}", content);

        let mut log_writer = path_to_hashes_log.unwrap();

        let result = log_writer.write(content.as_bytes());

        if result.is_err() {
            println!("Failed to record hash entry: {:?}", result);
        }
        let _ = log_writer.flush();
    }
}

pub fn init_path_to_obj_hash_file(metadata_folder_path: &str) -> Option<PathToHash> {
    if !Path::new(metadata_folder_path).exists() {
        return Option::None;
    }

    let path = format!("{}/path_to_hash.txt", metadata_folder_path);

    let file_contents = fs::read_to_string(&path);

    let mut path_to_hashes = PathToHash { entries: vec![] };

    if !Path::new(&path).exists() || file_contents.is_err() {
        let path_to_obj_log = File::create(&path);
        if path_to_obj_log.is_err() {
            return Option::None;
        }

        return Option::Some(path_to_hashes);
    }

    // TODO: use buffered readers instead
    let unwrapped_contents = file_contents.unwrap();
    let path_to_hashes_log = unwrapped_contents.lines();

    for path_to_hash_entry in path_to_hashes_log {
        let tokens = path_to_hash_entry.split(" ");

        let mut hash = "".to_string();
        let mut relative_filepath = "".to_string();

        for (index, token) in tokens.enumerate() {
            match index {
                0 => hash = token.to_string(),
                1 => relative_filepath = token.to_string(),
                _ => {}
            }
        }

        // storing relative file paths to file hashes
        path_to_hashes.entries.push(PathToHashEntry {
            hash,
            relative_filepath,
        });
    }

    return Option::Some(path_to_hashes);
}
