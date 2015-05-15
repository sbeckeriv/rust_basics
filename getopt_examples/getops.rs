//rustc 1.1.0-nightly (c4b23aec4 2015-04-29) (built 2015-04-28)
#![feature(rustc_private)]
#![feature(collections)]

extern crate getopts;
use std::env;
use std::process::exit;

fn main(){
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let options = [
        getopts::optopt("i", "", "Number Conversion", "INT"),
        getopts::optflag("h", "help", "Print help"),
        ];

    let commands = match getopts::getopts(args.tail(), &options){
        Ok(matches) => matches,
        Err(failure) => {
            println!("{}", failure);
            exit(1);
        }
    };

    if commands.opt_present("help"){
        println!("{} - demo getopts", program );
        println!("");
        println!("Usage:");
        println!(" {} [SHORT-OPTION]... [STRING]...", program);
        println!(" {} LONG-OPTION", program);
        println!("");
        let usage = getopts::usage("Display the options",
                                    &options);
        println!("{}", usage);
        exit(1);
    }

    let mut number_arg = 2u64;
    if commands.opt_present("i"){
        let number = commands.opt_str("i").unwrap();
        number_arg = match number.parse::<u64>(){
            Ok(r) => r,
            Err(_) =>exit(1)
        };
        println!("Number set:: {}", number);
    }
    println!("Converted Number :: {}", number_arg);

    if !commands.free.is_empty() {
    } else {

    };
}
