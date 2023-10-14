#![allow(unused)]
use std::process::Command;
use std::ffi::OsString;
use std::fs::File;
use std::{fs,thread,path::Path,env};
use std::env::{current_dir, set_current_dir};


//use std::path;
fn main() {
    read_files_in_folder();
}
fn read_files_in_folder(){

    let dir = current_dir().unwrap();
    for files in fs::read_dir(&dir).expect("{file:?} failed to open"){
        let file = &files.unwrap().file_name();
        let mut f = File::open(file).unwrap().metadata().expect("{file:?} failed to open");
            match f.is_dir() {
                true => {
                    let paths = [Path::new(&dir), Path::new(file)];
                    let file_path = env::join_paths(paths.iter()).unwrap();
                    println!("{file_path:?} is about to enter");
                    recursive(&file_path)

                },
                false => println!("it is not a directory")

            }
        run_command(&file);
    }
    

    println!("{dir:?}")
}

fn recursive(args: &OsString){
    for files in fs::read_dir(&args).expect("{files:?} failed to open"){
        let file = &files.unwrap().file_name();
        let mut f = File::open(file).unwrap().metadata().expect("{file:?} failed to open");
            match f.is_dir() {
                true => {
                    let paths = [Path::new(&file), Path::new(file)];
                    let file_path = env::join_paths(paths.iter()).unwrap();
                    println!("{file_path:?} is about to enter");
                    recursive(&file_path);

                },
                false => println!("it is not a directory")

            }
    }
}



fn run_command(args:&OsString){
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo This is windows"])
            .spawn()
            .expect("failed to execute process")
} else {
    Command::new("du")
        .arg("-hs")
        .arg(args)
        .spawn()
        .expect("failed to execute process")
    };
}

//TO DO
// 1. Reads all file in the system and displays the size
//  implement flags;
//      --f = a file part for a more condenced search
//      --s = a threshhold minimum filesize for the program to display the file