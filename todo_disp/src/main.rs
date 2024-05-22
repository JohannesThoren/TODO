
// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
mod args;
mod handle_args;
use handle_args::handle_filter;
use inline_colorization::*;

use std::{fs::File, io::BufReader};

use args::Args;
use clap::Parser;
use libtodo;

fn disp_term(files: Vec<libtodo::file::File>) {
    for f in files {
        println!("┌File: {color_green}{}{color_reset}", f.path);

        for (n, todo) in f.todos.clone().iter().enumerate() {
            if n == f.todos.len() - 1 {
                println!(
                    "└─ {}{:<8}{color_reset} L:{color_green}{:<6}{color_reset} - {}",
                    todo.priority.get_color(),
                    todo.priority.to_string(),
                    todo.line,
                    todo.description
                )
            }
            else {
                println!(
                    "├─ {}{:<8}{color_reset} L:{color_green}{:<6}{color_reset} - {}",
                    todo.priority.get_color(),
                    todo.priority.to_string(),
                    todo.line,
                    todo.description
                )
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    
    let f = File::open(args.input)?;
    let r = BufReader::new(f);
    let json: Vec<libtodo::file::File> = serde_json::from_reader(r)?;


    let filtered = handle_filter(json, args.filter);

    // TODO High : this is a test todo
    disp_term(filtered?);

    Ok(())
}
