
extern crate getopts;

use std::io;
use std::io::prelude::*;
use getopts::Options;
use std::env;



fn boxed(lines: Vec<String>, box_char: Vec<char>) {
    let mut first_line = String::new();
    let mut last_line = String::new();

    let max_line = lines.iter().fold(0, |acc, item| {
                    if item.len() > acc { item.len() } else { acc }
    });

    first_line.push(box_char[0]);
    last_line.push(box_char[1]);

    for _i in 0..max_line+2 {
        first_line.push(box_char[2]);
        last_line.push(box_char[2]);
    }

    first_line.push(box_char[3]);
    last_line.push(box_char[4]);

    println!("{}", first_line);
    for line in &lines {
        println!("{} {:width$} {}", box_char[5], line,
            box_char[5],width=max_line);
    }
    println!("{}", last_line);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_args(args: Vec<String>) -> Option<Vec<char>> {
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("c", "", "set box character", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }

    let char_opt = matches.opt_str("c");
    let box_char = match char_opt {
        Some(x) => vec![x.chars().next().unwrap(); 6],
        None =>  vec!['╔', '╚', '═', '╗', '╝', '║'],
    };

    Some(box_char)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let mut lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    match parse_args(args) {
        Some(x) => return boxed(lines, x),
        None    => return,
    }
}
