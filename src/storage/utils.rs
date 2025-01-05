use std::fs::File;
use std::path::Path;
use zip::write::{ExtendedFileOptions, FileOptions};
use zip::ZipWriter;

use std::io::{Read, Write};

use std::{fs, fs::read_dir, path::PathBuf};

pub fn zip_file(zipped_filepath: &Path, filepath_to_zip: &str) -> bool {
    let new_archive_result = File::create(zipped_filepath);

    if new_archive_result.is_err() {
        return false;
    }

    let new_archive = new_archive_result.unwrap();

    let mut zipper = ZipWriter::new(new_archive);

    let options: FileOptions<ExtendedFileOptions> =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Bzip2);

    let zipped_filename = zipped_filepath.file_name().unwrap().to_str().unwrap();

    let zip_start = zipper.start_file(zipped_filename, options);

    if zip_start.is_err() {
        println!("Error when trying to start zipping file.");
        return false;
    }

    let file_to_zip = File::open(filepath_to_zip);

    if file_to_zip.is_err() {
        return false;
    }

    let mut buffer = Vec::new();

    if File::read(&mut file_to_zip.unwrap(), &mut buffer).is_err() {
        return false;
    }

    let write_result = zipper.write_all(&buffer);

    if write_result.is_err() {
        return false;
    }

    if zipper.finish().is_err() {
        println!("Zipper finish threw error");
        return false;
    }

    println!(
        "zip_file, filepath to zip: {}. zipped filepath: {}",
        filepath_to_zip,
        zipped_filepath.to_str().unwrap()
    );

    return true;
}

pub fn hash_file(filepath: &str) -> Option<String> {
    let file = fs::read(filepath);

    if file.is_ok() {
        let file_contents = file.unwrap();

        let hash = sha256::digest(file_contents);

        return Option::Some(hash);
    }

    return Option::None;
}

pub fn recursive_dir_search(
    dir_path_to_search: &str,
    entry_name_to_find: &str,
    is_dir: bool,
) -> Option<PathBuf> {
    let current_dir_entries_option = read_dir(dir_path_to_search);

    if current_dir_entries_option.is_err() {
        println!(
            "RECURSIVE dir path to search: {}, couldn't not find entry name: {}",
            dir_path_to_search, entry_name_to_find
        );
        return Option::None;
    }

    let current_dir_entries = current_dir_entries_option.unwrap();

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

        if current_dir_entry_is_dir {
            return recursive_dir_search(
                dir_entry_path.to_str().unwrap(),
                entry_name_to_find,
                is_dir,
            );
        }
    }

    return Option::None;
}

// Pre condition: dir_path_to_search must exists
pub fn search_dir(
    dir_path_to_search: &str,
    entry_name_to_find: &str,
    is_dir: bool,
) -> Option<PathBuf> {
    let current_dir_entries_option = read_dir(dir_path_to_search);

    if current_dir_entries_option.is_err() {
        println!(
            "dir path to search: {}, couldn't not find entry name: {}",
            dir_path_to_search, entry_name_to_find
        );
        return Option::None;
    }

    let current_dir_entries = current_dir_entries_option.unwrap();

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

pub fn create_file(filename: &str, path: &str) -> Option<File> {
    let filepath = format!("{}/{}", path, filename);

    let new_file = File::create(filepath);

    return match new_file {
        Ok(_) => Option::Some(new_file.unwrap()),
        Err(_) => {
            println!("Create File failed, filename: {}, path: {}", filename, path);
            Option::None
        }
    };
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
