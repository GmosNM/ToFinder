use std::fs::{File, OpenOptions};
use std::io::prelude::*;


fn format_file(file_path: String) {
    let content: String = read_file(file_path.clone());
    let formatted_content = content.replace("\t", "    ");
    let mut file = File::create(file_path).expect("ERROR: could not create file");
    file.write_all(formatted_content.as_bytes())
        .expect("ERROR: could not write to file");
}


fn read_file(file_path: String) -> String {
    let mut file =  OpenOptions::new().read(true).open(file_path).expect("ERROR: could not read file");
    let metadata = file.metadata();
    let mut permissions = metadata.expect("error").permissions();
    permissions.set_readonly(true);
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        eprintln!("ERROR: could not read file: {}", e);
        std::process::exit(1);
    }
    return buffer;
}

fn add_todos_to_file(file_path: String) {
    let content: String = read_file(file_path);
    let lines: Vec<&str> = content.lines().collect();
    let mut file = OpenOptions::new().write(true).create(true).write(true).open("TODOS").unwrap();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("//todo") || line.contains("//TODO") || line.contains("// TODO") {
            let todo = format!("Line {}: {}\n", i + 1, line);
            file.write_all(todo.as_bytes())
                .expect("ERROR: could not write to file");
            println!("{}", todo);
        }
    }
}

fn read_dir(dir_path: String) {
    let paths = std::fs::read_dir(dir_path).expect("ERROR: could not read directory");
    for path in paths {
        let file_path = path.unwrap().path().display().to_string();
        add_todos_to_file(file_path.clone());
        format_file("TODOS".to_string());
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <dir_path>", args[0]);
        return;
    }

    let dir_path = &args[1];
    read_dir(dir_path.clone());
}
