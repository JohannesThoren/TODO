// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::prio::Priority;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub description: String,
    pub priority: Priority,
    pub line: usize,
}

const TODO_IDENTIFIER: &'static str = "todo";

impl Item {
    pub fn new(description: String, priority: Priority, line: usize) -> Item {
        Item {
            description,
            priority,
            line,
        }
    }


    pub fn from_comment(comment: &String, line: usize, comment_prefixes: Vec<&str>) -> Option<Item> {
        let split: Vec<String> = comment.split(":").map(|v| v.to_string()).collect();
        let todo: &String = &split[0];
        let todo_vec: Vec<String> = todo
            .split_whitespace()
            .filter(|v| !comment_prefixes.contains(v))
            .map(|v| v.to_string())
            .collect();

        if todo_vec.len() < 1 {
            return None
        }

        if todo_vec[0].to_ascii_lowercase() == TODO_IDENTIFIER {
            let mut prio = Priority::TBD;
            if todo_vec.len() > 1 {
                prio = Priority::from(&todo_vec[1]).unwrap();
            }

            if split.len() < 2 {
                return None;
            }

            let item = Item {
                description: split[1].clone().trim().to_string(),
                priority: prio,

                // fixing of by one
                line: line + 1,
            };

            return Some(item);
        }

        None
    }
}