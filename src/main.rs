use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::path::{PathBuf, Path};


fn format_file(file_path: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut buffer = String::new();

    for line in reader.lines() {
        let line = line?;
        let mut new_line = String::new();
        let last_char = ' ';

        for c in line.chars() {
            match (c, last_char == ' ') {
                ('\t', _) | (' ', false) => new_line.push(' '),
                _ => new_line.push(c),
            }
        }

        buffer.push_str(&new_line);
        buffer.push('\n');
    }

    let mut file = File::create(file_path)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

fn read_file(file_path: &Path) -> String {
    let mut file = OpenOptions::new().read(true).open(file_path).expect("ERROR: could not read file");
    let metadata = file.metadata().expect("error");
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        eprintln!("ERROR: could not read file: {}", e);
        std::process::exit(1);
    }
    buffer
}

<<<<<<< HEAD

=======
//todo: it prints all the todos but dose not add all of them to the file
>>>>>>> cf8ded67821033f47bd9b10b643950dc9da5d779
fn add_todos_to_file(file_path: &Path) {
    let mut todos_file = OpenOptions::new().append(true).open("TODOS").unwrap();
    let content = read_file(&file_path);
    let lines = content.lines();
    let mut todos = String::new();
    for (i, line) in lines.enumerate() {
        if line.contains("//todo") || line.contains("//TODO")
            || line.contains("// TODO") || line.contains("// todo") {
                let path_buf = PathBuf::from(file_path);
                let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                let file_path_str = file_path.to_str().unwrap();
                if file_name != "TODOS" {
                    let todo = format!("{}:{}: {}\n", file_path_str.to_string(), i + 1, line);
                    todos.push_str(&todo);
                }
        }
    }
    let mut todos_file_content = String::new();
    if let Ok(mut file) = File::open("TODOS") {
        file.read_to_string(&mut todos_file_content).unwrap();
    }
    let mut new_todos = String::new();
    for todo in todos.lines() {
       if !todos_file_content.contains(&todo.trim()) {
            new_todos.push_str(todo);
            new_todos.push('\n');
        }
    }
    if !new_todos.is_empty() {
        todos_file.write_all(new_todos.as_bytes()).unwrap();
        println!("{}", new_todos);
    }
}



fn read_dir(dir_path: &Path) {
    for entry in std::fs::read_dir(dir_path).expect("ERROR: could not read directory") {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_file() {
                add_todos_to_file(&entry.path());
            } else if entry.file_type().unwrap().is_dir() {
                read_dir(&entry.path());
            }
        }
    }

    format_file("TODOS").expect("ERROR: could not format the file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <dir_path_or_file>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    if path.is_dir() {
        read_dir(path);
    } else if path.is_file() {
        let mut todos_file = OpenOptions::new().create(true).write(true).open("TODOS").unwrap();
        todos_file.write_all(b"").expect("ERROR: could not clear file");
        add_todos_to_file(path);
        format_file(&args[1]).expect("ERROR: could not format the file");
    } else {
        println!("Invalid input. Please enter a valid directory or file path.");
    }
}
