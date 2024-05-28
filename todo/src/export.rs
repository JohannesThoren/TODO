/******************************************************************************
 * Copyright (c) 2024 Johannes Thorén. All rights reserved.                   *
 * SPDX-License-Identifier: BSD-4-Clause                                      *
 ******************************************************************************/
use std::fmt::format;
use std::fs;
use std::process::Output;
use clap::builder::Str;
use clap::Subcommand;
use libtodo::file::SourceFile;
use crate::display::parse_source_files;

#[derive(Debug, Subcommand)]
pub(crate) enum ExportTarget {
    MD,
    HTML,
    TEXT,
}

pub(crate) fn export(target: ExportTarget, input: String, filter: String, output: Option<String>) -> Result<(), std::io::Error> {
    let filtred = parse_source_files(input, filter)??;


    match target {
        ExportTarget::MD => {}
        ExportTarget::HTML => {}
        ExportTarget::TEXT => {
            if output.is_some() {
                fs::write(output.unwrap(), text(filtred)?).expect("TODO: panic message");
            } else {
                fs::write("todos.txt", text(filtred)?).expect("TODO: panic message");
            }
        }
    }
    Ok(())
}

pub fn text(source_files: Vec<SourceFile>) -> Result<String, std::io::Error> {
    let mut res = String::from("");

    for sf in source_files {
        res.push_str(format!("{}\n", sf.path).as_str());

        for todos in sf.todos {
            res.push_str(format!("\t- l:{:<5} {:<7} {} \n", todos.line, todos.priority.to_string(), todos.description).as_str())
        }
    }

    Ok(res)
}