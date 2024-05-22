// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    HIGH = 0,
    MEDIUM = 1,
    LOW = 3,
    TBD = 4,
}

impl Priority {
    pub fn from(s: &str) -> Option<Priority> {
        match s.to_lowercase().as_str() {
            "high" => Some(Priority::HIGH),
            "medium" => Some(Priority::MEDIUM),
            "low" => Some(Priority::LOW),
            "tbd" => Some(Priority::TBD),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Priority::HIGH => String::from("HIGH"),
            Priority::MEDIUM => String::from("MEDIUM"),
            Priority::LOW => String::from("LOW"),
            _ => String::from("TBD"),
        }
    }

    pub fn get_color(&self) -> &str {
        match &self {
            Priority::HIGH => "\x1B[31m",
            Priority::MEDIUM => "\x1B[33m",
            Priority::LOW => "\x1B[32m",
            _ => "\x1B[34m",
        }
    } 
}
