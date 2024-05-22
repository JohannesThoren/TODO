// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use inline_colorization::*;
use libtodo::{file::{self, File}, item::Item};

fn pretty_print_filter(parsed_filter: &Vec<String>) {
    if parsed_filter.len() > 0 {
        println!("┌Filter");
        for (n, pi) in parsed_filter.clone().into_iter().enumerate() {
            if n == (parsed_filter.len() - 1) {
                println!("└─{color_green}{}{color_reset}", pi)
            } else {
                println!("├─{color_green}{}{color_reset}", pi)
            }
        }
    }
}

fn parse_filter(filter_string: &String) -> Result<Vec<String>, std::io::Error> {
    let parsed_filter: Vec<String> = filter_string.split(" ").map(|v| v.to_string().to_ascii_uppercase()).collect();

    if (parsed_filter[0] != "".to_string()) {
        pretty_print_filter(&parsed_filter);
    }

    Ok(parsed_filter)
}

pub(crate) fn handle_filter(
    files: Vec<libtodo::file::File>,
    filter: String,
) -> Result<Vec<libtodo::file::File>, std::io::Error> {
    let filter: Vec<String> = parse_filter(&filter)?;

    let mut filtred_vec: Vec<File> = Vec::new();

    for f in files {
        let todos: Vec<Item> = f
            .todos
            .iter()
            .filter(|v| filter.contains(&v.priority.to_string()))
            .map(|v| v.clone())
            .collect();
        

        if todos.len() > 0 {
            filtred_vec.push(
                File { path: f.path, todos: todos }
            )
        }
    
    }
    

    Ok(filtred_vec)
}
