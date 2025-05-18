/******************************************************************************
 * Copyright (c) 2024 Johannes Thorén. All rights reserved.                   *
 * SPDX-License-Identifier: BSD-4-Clause                                      *
 ******************************************************************************/

mod display;
mod export;
mod scan;

use crate::display::display;
use crate::export::export;
use crate::scan::scan;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // TODO high: implement an export command.
    // the export command should allow the user to export the todos to html or md or raw text
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

    // todo medium : implement the functionality for the md and html targets
    EXPORT {
        #[command(subcommand)]
        target: export::ExportTarget,

        #[arg(short, long, default_value_t = String::from("high medium low tbd"))]
        filter: String,

        #[arg(short, long, default_value_t = String::from("todos.json"))]
        input: String,

        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    return match args.commands {
        Commands::SCAN {
            root,
            output,
            ignore,
            comment_prefixes,
            hidden,
            config,
        } => scan(root, &hidden, ignore, comment_prefixes, output, config),
        Commands::DISPLAY { filter, input } => display(input, filter),
        Commands::EXPORT {
            target,
            filter,
            input,
            output,
        } => export(target, input, filter, output),
    };
}
