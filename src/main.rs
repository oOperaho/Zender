use zender::Terminal;

use std::{self, env, fs, process};
use std::fs::{File, FileType};
use std::io::Error;
use std::time::SystemTime;

fn main() {
    let ger: Vec<String> = env::args().collect();

    let ctx = Terminal::new(&ger).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if ctx.flag == "help" {
        let h = help();
        println!("{}", h);
    } else if ctx.flag == "get" {
        let g = get(&ctx.text, &ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        println!("{:?}", g);
    } else if ctx.flag == "size" {
        let s = size(&ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        if s <= 1000 {
            println!("Size of '{}': {} bytes.", ctx.target, s);
        } else if s <= 1000000 {
            let n = format!("{num}", num=s);
            let res = two_nums(&n);
            println!("Size of '{}': {}.{}kb ({} bytes).", ctx.target, res[0], res[1], s);
        } else if s <= 1000000000 {
            let n = format!("{num}", num=s);
            let res = two_nums(&n);
            println!("Size of '{}': {}.{}mb ({} bytes).", ctx.target, res[0], res[1], s);
        } else if s <= 1000000000000 {
            let n = format!("{num}", num=s);
            let res = two_nums(&n);
            println!("Size of '{}': {}.{}gb ({} bytes).", ctx.target, res[0], res[1], s);
        } else {
            println!("Well, this file is really gigantic: {} bytes.", s);
        }
    } else if ctx.flag == "date" {
        let d = date(&ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        println!("This file was created in {:?}", d);
    } else if ctx.flag == "ftype" {
        let f = ftype(&ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        println!("This file has type {:?}", f);
    } else if ctx.flag == "see" {
        let s = see(&ctx.target).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1)
        });
        println!("Content of {}:\n{}", &ctx.target, s);
    } else if ctx.flag == "put" {
        put(&ctx.text, &ctx.target).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        });
        println!("Done. Run 'zender see' to check the results.");
    }
}

// Here, there are the definitions of each command that Zender can do
fn help() -> String {
    String::from("WELCOME TO ZENDER
                The pattern is: zender -flag- -file-

                Here's a nice command list:
                --------------------------
                help: Displays this message, of course
                get: Will search a word/text on a file for you
                    syntax: zender get my_word my_file.txt
                see: It prints the file's content
                put: Will write somethint to a file
                size: Will show you the size of the file
                date: Displays the file's date of creation
                ftype: Shows the type of the specified file")
}

fn get(s0: &str, s1: &str) -> Result<usize, Error> {
    let file = fs::read_to_string(s1)?;
    let mut v = Vec::new();

    for occ in file.lines() {
        if occ.contains(s0) {
            v.push(occ);
        }
    }

    Ok(v.len())
}

fn see(s: &str) -> Result<String, Error> {
    let file = fs::read_to_string(s)?;

    Ok(file)
}

fn put(s0: &str, s1: &str) -> Result<(), Error> {
    fs::write(s0, s1)?;

    Ok(())
}

fn size(s: &str) -> Result<u64, Error> {
    let file = File::open(s)?;
    file.sync_all()?;
    let size = file.metadata()?;

    Ok(size.len())
}

fn date(s: &str) -> Result<SystemTime, Error> {
    let file = File::open(s)?;
    file.sync_all()?;
    let date = file.metadata()?;

    date.created()
}

fn ftype(s: &str) -> Result<FileType, Error> {
    let file = File::open(s)?;
    file.sync_all()?;
    let ftype = file.metadata()?;

    Ok(ftype.file_type())
}

// Now these are functions to make the code more clean
fn two_nums(s: &str) -> Vec<&str> {
    let mut v = Vec::new();
    let a1 = &s[0..1];
    let a2 = &s[1..2];
    v.push(a1);
    v.push(a2);
    v
}
