use cicl::Terminal;

use std::env;
use std::fs;
use std::process;

fn main() {
    let ger: Vec<String> = env::args().collect();

    let ctx = cicl::Terminal::new(&ger).unwrap_or_else(|_err| {
        println!("Error: {}", _err);
        process::exit(1);
    });

    if ctx.flag == "help" {
        let h = help();
        println!("{}", h);
    } else if ctx.flag == "size" {
        let s = size(&ctx.target);
        println!("{}", s);
    }
    let counter = cicl::Terminal::new(&ger).unwrap_or_else(|err| {
        println!("Couldn't parse arguments!");
        process::exit(1);
    });

    if counter.flag == "get" {
        let g = get(&counter.text, &counter.target);
        println!("{}", g);
    }
}

fn help() -> String {
    String::from("WELCOME TO ZENDER
                The pattern is: zender -flag- -file-

                Here's a nice command list:
                --------------------------
                help: Displays this message, of course
                get: Will search a word/text on a file for you
                    syntax: zender get my_word my_file.txt
                szof: Will show you the size of the file")
}

fn get(s0: &str, s1: &str) -> usize {
    let file = fs::read_to_string(s1).expect("Could not read the file!");
    let mut v = Vec::new();

    for occ in file.lines() {
        if occ.contains(s0) {
            v.push(occ);
        }
    }

    v.len()
}

fn size(_s: &str) -> f64 {
    7.0
}