#![allow(unused)]

use std::{fs, thread, path::PathBuf, env, time, process};
use std::env::{current_dir, set_current_dir};
use sizer::entry;
use clap::{Parser, Subcommand, Command, arg};
use log;


#[derive(Parser,Default,Debug)]
#[command(author = "Buki", version = "0", about = "This tool searches for files bigger than 100mb in a given directory", long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}



fn main(){
    env_logger::init();
    let matches = Command::new("Sizer")
    .version("1.0")
    .author("Buki O. <buki.offor@gmail.com>")
    .about("This tool searches for files bigger than 100mb in a given directory or the whole file system if no filepath is given")
    .arg(arg!(-p --path <VALUE> "The filepath that you want to perform a search on").required(false))
    .arg(arg!(-s --size <VALUE> "The size of the file in megabyte that you want to log when found, defaults to 100mb by default").required(false))
    .get_matches();

    let path = matches.get_one::<String>("path");
    match path {
        Some(path) => {            
            let size: Option<&String> = matches.get_one::<String>("size");
            
            let home_dir = PathBuf::from(path);
            entry(&home_dir, size);
            
        }
        None => {
            let size: Option<&String> = matches.get_one::<String>("size");
            //gets the current directory
            let root_dir = PathBuf::from("/");
            entry(&root_dir, size);
        }
    }

    
}












fn second_main(){
    
    env_logger::init();
    let args:Vec<String> = env::args().collect();
    println!("{args:?}");
    if args.len() > 2 {
        log::error!("wrong arguement, please use file path as an arguement");
        process::exit(1)
    }else if args.len() == 2 {
        let mut args = args.iter();
        args.next();
        // reads the home directory
        let home_dir = PathBuf::from(args.next().unwrap());
        //test_read_files(&home_dir);
    }else{
        //gets the current directory
        let current_dir = current_dir().unwrap();
        //test_read_files(&current_dir);
        //println!("{:?}",home_dir)
    }
    
}