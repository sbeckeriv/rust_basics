//rustc 1.1.0-nightly (c4b23aec4 2015-04-29) (built 2015-04-28)

#![feature(rustc_private)]
#![feature(collections)]
extern crate getopts;

use std::io::prelude::*;
use std::env;
use std::process::exit;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Counts{
    words: u64,
    bytes: u64,
    lines: u64,
    m: u64
}
impl Counts{
    fn new(w: u64, l: u64, b: u64) -> Counts{
        Counts {words: w, bytes: b, lines:l , m:3}
    }
}

fn process_files(files: &Vec<String>, options: HashSet<&str>){
    let mut file_data = HashMap::<String, Counts>::new();

    for file in files{
        let file_name = file.clone();
        let mut f = match File::open(file) {
            Err(why) => panic!("couldn't open {} - {}", file_name, why),
            Ok(f) => f
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {} - {}", file_name, why),
            Ok(d) => d,
        };
        println!("{}", s);
        let w: u64 = s.split_whitespace().count() as u64;
        let l: u64 = s.lines().count() as u64;
        let b: u64 = s.into_bytes().len() as u64;
        let details = Counts::new(w, l, b);

        let file_copy = file.clone();
        file_data.insert(file_copy.to_string(), details);
    }
    println!("{:?}", file_data)
}


fn print_help(){
    println!("wc -- word, line, character, and byte count");
    println!("");
    println!("Usage:");
    println!("    wc [-clmw] [file ...]");
    exit(0);
}

fn main(){
    let mut flags = HashSet::new();
    let v: Vec<&str> = vec!("words", "lines",
                            "btyes");

    let default_flags: HashSet<&str> = v.into_iter().collect();


    let args: Vec<String> = env::args().collect();
    let options = [
        getopts::optopt("m", "", "Count Ms", "m"),
        getopts::optopt("w", "", "Count Words", "w"),
        getopts::optopt("l", "", "Count Lines", "l"),
        getopts::optopt("c", "", "Count Bytes", "c"),
        getopts::optflag("h", "help", "Print help")
            ];

    let commands = match getopts::getopts(args.tail(), &options){
        Ok(matches) => matches,
        Err(failure) => {
            println!("{}", failure);
            exit(1);
        }
    };

    if commands.opt_present("help"){
        print_help();
    }

    if commands.opt_present("w"){
        flags.insert("words");
    }

    if !commands.free.is_empty() {
        if flags.is_empty(){
            process_files(&commands.free, default_flags);
        } else{
            process_files(&commands.free, flags);
        }
    } else {
        print_help();

    };
}
