// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, default_value_t = String::from("todos.json"))]
    pub(crate) input: String,

    #[arg(short, long ,default_value_t= String::from("high medium low tbd"))]
    pub(crate) filter: String

}
