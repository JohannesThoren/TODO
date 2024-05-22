// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
use inline_colorization::*;

pub type IgnoreVector = Vec<String>;


pub(crate) fn parse_ignore(ignore_string: &String) -> Result<Vec<String>, std::io::Error> {
    let parsed_ignore: Vec<String> = ignore_string.split(" ").map(|v| v.to_string()).collect();


    if parsed_ignore[0] != "".to_string() {
        pretty_print_ignore(&parsed_ignore);
    }

    Ok(parsed_ignore)
}

fn pretty_print_ignore(parsed_ignore: &Vec<String>) {
    if parsed_ignore.len() > 0 {
        println!(
            "┌Ignoring {color_green}{}{color_reset} folders",
            parsed_ignore.len()
        );
        for (n, pi) in parsed_ignore.clone().into_iter().enumerate() {
            if n == (parsed_ignore.len() - 1) {
                println!("└─{color_green}{}{color_reset}", pi)
            } else {
                println!("├─{color_green}{}{color_reset}", pi)
            }
        }
    }
}
