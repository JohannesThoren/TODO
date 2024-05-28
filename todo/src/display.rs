use std::fs::File;
use std::io::BufReader;
use libtodo::file::SourceFile;
use libtodo::item::Item;
use inline_colorization::*;
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


// TODO medium : fixme
fn parse_filter(filter_string: &String) -> Result<Vec<String>, std::io::Error> {
    let parsed_filter: Vec<String> = filter_string.split(" ").map(|v| v.to_string().to_ascii_uppercase()).collect();

    if parsed_filter[0] != "".to_string() {
        pretty_print_filter(&parsed_filter);
    }

    Ok(parsed_filter)
}

pub(crate) fn handle_filter(
    files: Vec<libtodo::file::SourceFile>,
    filter: String,
) -> Result<Vec<libtodo::file::SourceFile>, std::io::Error> {
    let filter: Vec<String> = parse_filter(&filter)?;

    let mut filtred_vec: Vec<SourceFile> = Vec::new();

    for f in files {
        let todos: Vec<Item> = f
            .todos
            .iter()
            .filter(|v| filter.contains(&v.priority.to_string()))
            .map(|v| v.clone())
            .collect();


        if todos.len() > 0 {
            filtred_vec.push(
                SourceFile { path: f.path, todos: todos }
            )
        }
    }


    Ok(filtred_vec)
}


pub(crate) fn display(input: String, filter: String) -> Result<(), std::io::Error> {
    let f = File::open(input)?;
    let r = BufReader::new(f);
    let json: Vec<libtodo::file::SourceFile> = serde_json::from_reader(r)?;


    let filtered = handle_filter(json, filter);

    for f in filtered? {
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
            } else {
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
    Ok(())
}