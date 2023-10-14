use std::fs;
use std::process::Command;
//use std::path;
fn main() {
    read_files_in_folder();
    run_command();
}

fn read_files_in_folder(){
    for files in fs::read_dir("/").unwrap(){
        let file = files.unwrap().file_name();
        println!("{file:?}")
    }
}
fn run_command(){
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo This is windows"])
            .spawn()
            .expect("failed to execute process")
} else {
    Command::new("du")
        .arg("-s")
        .arg("-h")
        .spawn()
        .expect("failed to execute process")
    };
}

//TO DO
// 1. Reads all file in the system and displays the size
//  implement flags;
//      --f = a file part for a more condenced search
//      --s = a threshhold minimum filesize for the program to display the file