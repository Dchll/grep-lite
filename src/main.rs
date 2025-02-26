use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Parser, Debug)]
#[command(
    version = "0.0.1",
    about = "searches for patterns",
    long_about = "by 多吃韩66"
)]
struct Args {
    #[arg(help = "The pattern to search for")]
    pattern: String,
    #[arg(help = "File to search", required = false)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let input = args.input;

    let re = Regex::new(pattern.as_str()).unwrap();

    if let Some(input) = input {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader,re);
    } else { 
        println!("请输入要查找的文本");
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader,re);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        if let Ok(line) = line_ {
            let contains_substring = re.find(&line);
            match contains_substring {
                None => (),
                Some(_) => println!("{} ({} bytes)", line, line.len()),
            }
        }
    }
}
