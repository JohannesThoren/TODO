use std::fs;
use libtodo::dir::read_dir;
use libtodo::file::SourceFile;
use inline_colorization::*;

pub(crate) fn scan(path: String, allow_hidden: &bool, ignore: String, comment_prefixes: String, output: String, config: bool) -> Result<(), std::io::Error> {
    let ignore: Vec<String> = ignore.split(" ").map(|v| v.to_string()).collect();

    let comment_prefixes = comment_prefixes.split(" ").map(|v| v).collect();

    let files = read_dir(path, &ignore, allow_hidden)?;
    let mut source_files: Vec<SourceFile> = Vec::new();

    for file in files {
        let sf = SourceFile::from_path(&file, &comment_prefixes);

        if sf.is_some() {
            source_files.push(sf.clone().unwrap());
        }
    }

    let source_files: Vec<SourceFile> = source_files.iter().filter(|s| s.todos.len() > 0).map(|s| s.clone()).collect();

    let j = serde_json::to_string(&source_files)?;

    let _ = fs::write(output, j)?;

    if config {
        print_config(&ignore, comment_prefixes, allow_hidden);
    }

    Ok(())
}

fn print_config(ignore: &Vec<String>, comment_prefixes: Vec<&str>, allow_hidden: &bool) {
    println!("┌{color_blue}Config{color_reset}");

    print_ignore_list(&ignore);

    print_comment_prefix_list(&comment_prefixes);

    if !allow_hidden {
        println!("└{color_blue}Allow Hidden{color_reset}: {color_red}{}{color_reset}", allow_hidden);
    } else {
        println!("└{color_blue}Allow Hidden{color_reset}: {color_green}{}{color_reset}", allow_hidden);
    }
}

fn print_ignore_list(ignore: &Vec<String>) {
    if ignore.len() > 1 {
        println!("├┬{color_blue}Ignoring{color_reset}");
        for (n, i) in ignore.iter().enumerate() {
            let mut box_char = "├";
            if n == ignore.len() - 1 {
                box_char = "└";
            }
            println!("│{box_char}─{color_green}{}{color_reset}", i);
        }
    }
}

fn print_comment_prefix_list(comment_prefix: &Vec<&str>) {
    println!("├┬{color_blue}Comment Prefixes{color_reset}");
    for (n, cp) in comment_prefix.iter().enumerate() {
        let mut box_char = "├";
        if n == comment_prefix.len() - 1 {
            box_char = "└";
        }
        println!("│{box_char}─{color_green}\"{}\"{color_reset}", cp);
    }
}