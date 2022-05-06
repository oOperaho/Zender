use zender::Terminal;

use std::{self, env, fs, process};
use std::fs::File;
use std::io::Error;

fn main() {
    let ger: Vec<String> = env::args().collect();

    let ctx = Terminal::new(&ger).unwrap_or_else(|_err| {
        println!("Error: {}", _err);
        process::exit(1);
    });

    if ctx.flag == "help" {
        let h = help();
        println!("{}", h);
    } else if ctx.flag == "get" {
        let g = get(&ctx.text, &ctx.target);
        println!("{}", g);
    } else if ctx.flag == "size" {
        let s = size(&ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        println!("Size of '{}': {:?} bytes.", ctx.target, s);
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
                size: Will show you the size of the file")
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

fn size(s: &str) -> Result<u64, Error> {
    let file = File::open(s)?;
    file.sync_all()?;
    let size = file.metadata()?;

    Ok(size.len())
}