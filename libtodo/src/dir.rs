/******************************************************************************
 * Copyright (c) 2024 Johannes Thorén. All rights reserved.                   *
 * SPDX-License-Identifier: BSD-4-Clause                                      *
 ******************************************************************************/

use std::fs;
use std::fs::ReadDir;
use std::io::Error;

pub struct DirEntries {
    files: Vec<String>,
    dirs: Vec<String>,
}

/// Reads a directory recursively and returns all files as a vector of Strings
pub fn read_dir(path: String, ignore: &Vec<String>, allow_hidden: &bool) -> Result<Vec<String>, std::io::Error> {
    let mut content: Vec<String> = Vec::new();

    if ignore.contains(&path) {
        return Ok(content);
    }

    let dir = fs::read_dir(path)?;

    let entries = get_children(dir, allow_hidden);

    read_child_dirs(&ignore, &allow_hidden, &mut content, entries?)?;
    Ok(content)
}

/// helper function for read_dir
fn read_child_dirs(ignore: &&Vec<String>, allow_hidden: &bool, content: &mut Vec<String>, mut entries: DirEntries) -> Result<(), Error> {
    content.append(&mut entries.files);

    if entries.dirs.len() > 0 {
        for d in entries.dirs {
            content.append(&mut read_dir(d, &ignore, allow_hidden)?)
        }
    }

    Ok(())
}

/// helper function for read_dir
fn get_children(dir: ReadDir, allow_hidden: &bool) -> Result<DirEntries, std::io::Error> {
    let mut dir_entries = DirEntries { files: vec![], dirs: vec![] };

    for entry in dir {
        let entry = entry?;
        let entry_path = entry.path().to_str().unwrap().to_string();
        let entry_name = entry.file_name().to_str().unwrap().to_string();

        if entry_name.starts_with(".")
            && !allow_hidden
        {
            continue;
        }

        if entry.metadata()?.is_dir() {
            dir_entries.dirs.push(entry_path.clone())
        }

        if entry.metadata()?.is_file() || entry.metadata()?.is_symlink() {
            dir_entries.files.push(entry_path.clone())
        }
    }

    Ok(dir_entries)
}