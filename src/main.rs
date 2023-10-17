#![allow(unused)]

use std::{fs, thread, path::PathBuf, env, time, process};
use std::env::{current_dir, set_current_dir};
use sizer::entry;
use clap::{Parser, Subcommand, Command, arg};
use log;


/// This is the main function of a Rust program that searches for files bigger than 100mb in a given directory or the whole file system if no filepath is given.
///
/// # Example Usage
/// ```
/// // Run the program with a specific directory path
/// $ ./sizer -p /path/to/directory
///
/// // Run the program without specifying a directory path (searches the whole file system)
/// $ ./sizer
///
/// // Run the program with a specific directory path and custom file size threshold
/// $ ./sizer -p /path/to/directory -s 200
/// ```
///
/// # Arguments
/// - `matches`: A `clap::ArgMatches` object that contains the parsed command line arguments.
///
/// # Flow
/// 1. Initialize the logger for logging purposes.
/// 2. Parse the command line arguments using `clap` to get the `matches` object.
/// 3. Get the value of the `path` argument from the `matches` object.
/// 4. If a `path` value is provided, convert it to a `PathBuf` object and assign it to `home_dir`.
/// 5. Get the value of the `size` argument from the `matches` object.
/// 6. If a `size` value is provided, assign it to `size` as an `Option<&String>`.
/// 7. If a `path` value is provided, call the `entry` function from the `sizer` library with `home_dir` and `size` as arguments.
/// 8. If no `path` value is provided, create a `PathBuf` object for the root directory ("/") and assign it to `root_dir`.
/// 9. Call the `entry` function from the `sizer` library with `root_dir` and `size` as arguments.
///
/// # Outputs
/// None. The function performs file size calculations and logging based on the provided command line arguments.
fn main(){
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let matches = Command::new("Sizer")
    .version("1.0")
    .author("Buki O. <ebuka2264@yahoo.com>")
    .about("This tool searches for files bigger than 100mb in a given directory or the whole file system if no filepath is given")
    .arg(arg!(-p --path <VALUE> "The filepath that you want to perform a search on").required(false))
    .arg(arg!(-s --size <VALUE> "The size of the file in megabyte that you want to log when found, defaults to 100mb by default").required(false))
    .get_matches();

    let path = matches.get_one::<String>("path");
    match path {
        Some(path) => {            
            let size: Option<&String> = matches.get_one::<String>("size");
            if size.unwrap().parse::<i64>().is_err(){
                log::error!("Invalid size arguement defaulting to 100mb");
                let home_dir = PathBuf::from(path);
                entry(&home_dir, None);
            }else{            
                let home_dir = PathBuf::from(path);
                entry(&home_dir, size);
            
        }}
        None => {
            let size: Option<&String> = matches.get_one::<String>("size"); // make sure its a valid number
            if size.unwrap().parse::<i64>().is_err(){
                log::error!("Invalid size arguement defaulting to 100mb");
                let root_dir = PathBuf::from("/");
                entry(&root_dir, None);
            }else{            
                //gets the current directory
                let root_dir = PathBuf::from("/");
                entry(&root_dir, size);
            }
        }
    }
    
}
















fn old_main(){
    
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