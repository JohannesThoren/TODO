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
    pub file: String,
}

impl Item {
    pub fn new(description: String, priority: Priority, line: usize, file: String) -> Item {
        Item {
            description,
            priority: priority,
            line,
            file,
        }
    }
}