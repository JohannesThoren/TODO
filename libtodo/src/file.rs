// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};

use crate::item::Item;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: String,
    pub todos: Vec<Item>,
}
