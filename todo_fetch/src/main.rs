mod args;
mod handle_args;
use std::fs;

use args::Args;
use clap::Parser;
use handle_args::{parse_ignore, IgnoreVector};
use inline_colorization::*;
use libtodo::{file::File, item::Item, prio::Priority};
const COMMENT_CHAR: [&str; 2] = ["//", "#"];

fn get_dir_content(
    path: String,
    ignore_vec: IgnoreVector,
    hidden: bool,
) -> Result<Vec<String>, std::io::Error> {
    let mut content: Vec<String> = Vec::new();

    if ignore_vec.contains(&path) {
        return Ok(content);
    }

    let dir = fs::read_dir(path)?;

    let mut entries = get_children(dir, hidden)?;

    if entries.is_some() {
        let mut entries = entries.unwrap();

        content.append(&mut entries.0);

        if entries.1.len() > 0 {
            for d in entries.1 {
                content.append(&mut get_dir_content(d, ignore_vec.clone(), hidden)?)
            }
        }
    }
    Ok(content)
}

fn get_children(
    dir: fs::ReadDir,
    hidden: bool,
) -> Result<Option<(Vec<String>, Vec<String>)>, std::io::Error> {
    let mut files: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();

    for dir_entry in dir {
        let de = dir_entry?;

        if de
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .starts_with(".")

            && !hidden
        {
            continue;
        }


        if de.metadata()?.is_dir() {
            dirs.push(de.path().to_str().unwrap().to_string())
        }

        if de.metadata()?.is_file() || de.metadata()?.is_symlink() {
            files.push(de.path().to_str().unwrap().to_string())
        }
    }

    Ok(Some((files, dirs)))
}

fn get_todos_from_file(path: String) -> Result<File, std::io::Error> {
    let lines = read_lines(path.clone());
    let comments = get_comments(lines);
    let mut todos: Vec<Item> = Vec::new();

    for comment in comments {
        let parsed = parse_todo(&comment.0, comment.1, path.clone());
        if parsed.is_some() {
            todos.push(parsed.unwrap());
        }
    }

    let file: File = File {
        path: path,
        todos: todos,
    };

    Ok(file)
}

fn parse_todo(comment: &String, line: usize, file: String) -> Option<Item> {
    let split: Vec<String> = comment.split(":").map(|v| v.to_string()).collect();
    let todo: &String = &split[0];
    let todo_vec: Vec<String> = todo
        .split_whitespace()
        .filter(|v| !COMMENT_CHAR.contains(v))
        .map(|v| v.to_string())
        .collect();

    if todo_vec[0].to_ascii_lowercase() == "todo" {
        let mut prio = Priority::TBD;
        if todo_vec.len() > 1 {
            prio = Priority::from(&todo_vec[1]).unwrap();
        }

        let item = Item {
            description: split[1].clone().trim().to_string(),
            priority: prio,

            // fixing of by one
            line: line + 1,
            file: file,
        };

        return Some(item);
    }

    None
}

fn get_comments(lines: Vec<String>) -> Vec<(String, usize)> {
    let mut comments: Vec<(String, usize)> = Vec::new();

    for line in lines.iter().enumerate() {
        let l = clean_line(line.1.to_string());
        if is_comment(&l) {
            comments.push((l, line.0));
        }
    }

    comments
}

fn clean_line(line: String) -> String {
    let clean = line.trim();
    clean.to_string()
}

fn is_comment(line: &String) -> bool {
    for cc in COMMENT_CHAR {
        if line.starts_with(cc) {
            return true;
        }
    }

    return false;
}

fn read_lines(path: String) -> Vec<String> {
    let lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    return lines;
}

fn print_summary(files: &Vec<File>) {
    for file in files {
        let high: Vec<Item> = file
            .todos
            .iter()
            .filter(|t| t.priority == Priority::HIGH)
            .map(|t| t.clone())
            .collect();

        let meium: Vec<Item> = file
            .todos
            .iter()
            .filter(|t| t.priority == Priority::MEDIUM)
            .map(|t| t.clone())
            .collect();

        let low: Vec<Item> = file
            .todos
            .iter()
            .filter(|t| t.priority == Priority::LOW)
            .map(|t| t.clone())
            .collect();

        let tbd: Vec<Item> = file
            .todos
            .iter()
            .filter(|t| t.priority == Priority::TBD)
            .map(|t| t.clone())
            .collect();

        println!("File {color_green}{}{color_reset} contains {color_green}{}{color_reset} High, {color_green}{}{color_reset} Medium, {color_green}{}{color_reset} Low and {color_green}{}{color_reset} TBD TODO items",file.path, high.len(), meium.len(), low.len(), tbd.len())
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let ignore = parse_ignore(&args.ignore)?;

    let allow_hidden = args.hidden;

    if allow_hidden {
        println!("Program will {color_red}NOT{color_reset} skip hidden items")
    }


    let content = get_dir_content(args.dir, ignore, allow_hidden);
    // println!("{:?}", content);

    let mut files: Vec<File> = Vec::new();

    for file in content? {
        let f = get_todos_from_file(file);
        files.push(f?);
    }

    files = files
        .iter()
        .filter(|f| f.todos.len() > 0)
        .map(|f| f.clone())
        .collect();

    // TODO : check if this works
    print_summary(&files);

    let j = serde_json::to_string(&files)?;

    let mut output = "todos.json".to_string();

    if !args.output.is_empty() {
        output = args.output;
    }

    let _ = fs::write(output, j);

    Ok(())
}
