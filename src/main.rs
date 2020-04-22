use std::io;
use std::io::prelude::*;

fn boxed(lines: Vec<String>) {
    let mut max_line = 0;
    let mut first_line = String::new();
    let mut last_line = String::new();

    for line in &lines {
        if max_line < line.len() {
            max_line = line.len();
        }
    }

    first_line.push('╔');
    last_line.push('╚');

    for _i in 0..max_line+2 {
        first_line.push('═');
        last_line.push('═');
    }

    first_line.push('╗');
    last_line.push('╝');

    println!("{}", first_line);
    for line in &lines {
        println!("║ {:width$} ║", line, width=max_line);
    }
    println!("{}", last_line);
}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }
    boxed(lines);
}
