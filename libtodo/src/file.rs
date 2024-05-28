// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::fs;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};

use crate::item::Item;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceFile {
    pub path: String,
    pub todos: Vec<Item>,
}

impl SourceFile {
    pub fn from_path(path: &String, comment_prefixes: &Vec<&str>) -> Option<Self> {
        let lines = read_lines(path.clone());

        if lines.is_none() {
            return None;
        }

        let comments = get_comments(lines.unwrap(), comment_prefixes.clone());
        let mut todos: Vec<Item> = Vec::new();

        for comment in comments {
            let parsed = Item::from_comment(&comment.0, comment.1, comment_prefixes.clone());
            if parsed.is_some() {
                todos.push(parsed.unwrap());
            }
        }

        Some(SourceFile { path: path.to_string(), todos })
    }
}

fn is_comment(line: &String, comment_prefixes: Vec<&str>) -> bool {
    for cc in comment_prefixes {
        if line.starts_with(cc) {
            return true;
        }
    }

    return false;
}

fn clean_line(line: String) -> String {
    let clean = line.trim();
    clean.to_string()
}

fn get_comments(lines: Vec<String>, comment_prefixes: Vec<&str>) -> Vec<(String, usize)> {
    let mut comments: Vec<(String, usize)> = Vec::new();

    for line in lines.iter().enumerate() {
        let l = clean_line(line.1.to_string());
        if is_comment(&l, comment_prefixes.clone()) {
            comments.push((l, line.0));
        }
    }

    comments
}

fn read_lines(path: String) -> Option<Vec<String>> {
    let f = fs::File::open(path.clone()).ok()?;
    let reader = BufReader::new(f);

    let mut lines = Vec::new();

    for l in reader.lines() {
        lines.push(l.ok()?);
    }

    return Some(lines);
}