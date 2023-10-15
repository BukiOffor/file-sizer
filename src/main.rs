#![allow(unused)]
use std::io::ErrorKind;
use std::process::Command;
use std::ffi::OsString;
use std::fs::{File, Metadata};
use std::{fs, fs::metadata, thread, path::Path, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use std::mem::drop;



fn main() {
    read_files_in_folder();
}

fn read_files_in_folder(){

    let path = PathBuf::from("/");
    let dir = current_dir().unwrap();
    for files in fs::read_dir(&path).expect("{file:?} failed to open"){
        let meta_data = files.as_ref().unwrap().metadata().unwrap();
        let file = &files.as_ref().unwrap().path();  
            match meta_data.is_dir() {
                true => {
                    let path = Path::new(&dir).join(file);
                    println!("{path:?} is about to enter");
                    test_recur(&path);
                },
                false => {
                    let result = run_command(&file);
                    match result {
                        Ok(Some(_)) => (),
                        Ok(None) => (),
                        Err(e) => panic!("Unexpected error: {}", e),
                    }

            }
    }
    
}


fn test_recur(args: &PathBuf){
    for files in fs::read_dir(&args).expect("failed to open"){
        let meta_data = files.as_ref().unwrap().metadata().unwrap();
        let file = &files.as_ref().unwrap().path();
        match meta_data.is_dir(){
            true =>{
                let file = &files.unwrap().path();
                println!("{file:?}");
                test_recur(file)
            },
            false => {
               match run_command(file) {
                Ok(Some(_)) => (),
                Ok(None) => {
                    continue
                },
                Err(e) => panic!("Unexpected error: {}", e),
               }
            }
        }
    }
}


fn run_command(args:&PathBuf)->std::io::Result<Option<()>>{

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo This is windows"])
            .output()
    } else {
    Command::new("du")
        .arg("-hs")
        .arg(args)
        .output()            
    };
    println!("{output:?}");
    match output {
        Ok(output) => {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);
                println!("{}", s);
            }
            Ok(Some(()))
        },
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                println!("Permission to check this file denied");
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

}