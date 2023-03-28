use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::path::Path;

fn format_file(file_path: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut buffer = String::new();

    for line in reader.lines() {
        let line = line?;
        let mut new_line = String::new();
        let mut last_char = ' ';

        for c in line.chars() {
            if c == '\t' {
                new_line.push(' ');
            } else if c == ' ' && last_char != ' ' {
                new_line.push(c);
            } else if c != ' ' {
                new_line.push(c);
            }
            last_char = c;
        }

        buffer.push_str(&new_line);
        buffer.push('\n');
    }

    let mut file = File::create(file_path)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

fn read_file(file_path: &str) -> String {
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

fn add_todos_to_file(file_path: &str, mut todos_file: &File) {
    let content = read_file(&file_path);
    let lines = content.lines();
    todos_file.write_all(b"").expect("ERROR: no file");
    for (i, line) in lines.enumerate() {
        match line {
            line if line.contains("//todo") || line.contains("//TODO")
                || line.contains("// TODO") || line.contains("// todo") => {
                let path_buf = PathBuf::from(file_path);
                let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                let todo = format!("{}:{}: {}\n", file_name, i + 1, line);
                todos_file.write_all(todo.as_bytes())
                    .expect("ERROR: could not write to file");
                println!("{}", todo);
            },
            _ => (),
        }
    }
}

fn read_dir(dir_path: &str) {
    let mut todos_file = OpenOptions::new().append(false).create(true).write(true).open("TODOS").unwrap();
    for entry in std::fs::read_dir(dir_path).expect("ERROR: could not read directory") {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_file() {
                add_todos_to_file(&entry.path().display().to_string(), &mut todos_file);
            } else if entry.file_type().unwrap().is_dir() {
                read_dir(&entry.path().display().to_string());
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
        read_dir(&args[1]);
    } else if path.is_file() {
        let mut todos_file = OpenOptions::new().append(true).create(true).write(true).open("TODOS").unwrap();
        todos_file.write_all(b"").expect("ERROR: could not clear file");
        add_todos_to_file(&args[1], &mut todos_file);
        format_file(&args[1]).expect("ERROR: could not format the file");
        println!("File formatted successfully!");
    } else {
        println!("Invalid input. Please enter a valid directory or file path.");
    }
}
