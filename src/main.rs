use std::io;
use std::io::prelude::*;

fn boxed(lines: Vec<String>) {
    let mut first_line = String::new();
    let mut last_line = String::new();

    let max_line = lines.iter().fold(0, |acc, item| {
                    if item.len() > acc { item.len() } else { acc }
    });

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
