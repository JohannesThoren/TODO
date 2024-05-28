// Copyright 2024 Johannes Thorén. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

mod scan;
mod display;

use clap::{Parser, Subcommand};
use crate::display::display;
use crate::scan::scan;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    SCAN {
        #[arg(short, long, default_value_t = String::from("."))]
        root: String,

        #[arg(short, long, default_value_t = String::from("todos.json"))]
        output: String,

        #[arg(short, long, default_value_t = String::from(""))]
        ignore: String,

        #[arg(short, long, default_value_t = String::from("// /* * #"))]
        comment_prefixes: String,

        #[arg(long)]
        hidden: bool,

        #[arg(long)]
        config: bool,
    },
    DISPLAY {
        #[arg(short, long, default_value_t = String::from("high medium low tbd"))]
        filter: String,

        #[arg(short, long, default_value_t = String::from("todos.json"))]
        input: String,
    },
}


fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    return match args.commands {
        Commands::SCAN { root, output, ignore, comment_prefixes, hidden, config } => {
            scan(root, &hidden, ignore, comment_prefixes, output, config)
        }
        /*
        * todo high: fix display!!!
        */
        Commands::DISPLAY { filter, input } => {
            display(input, filter)
        }
    };


    Ok(())
}
