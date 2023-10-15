#![allow(unused)]
use std::io::ErrorKind;
use std::ops::Deref;
use std::process::Command;
use std::ffi::OsString;
use std::fs::{File, Metadata};
use std::{fs, fs::metadata, thread, path::Path, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use std::mem::drop;



fn main() {
    read_files()
}








fn test_read_files(path:&PathBuf){
    if fs::read_dir(&path).is_ok() {
        for files in fs::read_dir(&path).expect("{file:?} failed to open"){
            let meta_data = files.as_ref().unwrap().metadata().unwrap();
            let file = &files.as_ref().unwrap().path();  
            match meta_data.is_dir() {
                true => {
                    test_recur(&file)
                },
                false => {
                   run_command(&meta_data, file) 
                }
        }

    }
}   
    
    
    
    
    
    
    // let file: Result<fs::ReadDir, std::io::Error> = fs::read_dir(path);
    // match file {
    //     Ok(file) => {
    //         Ok(Some(file))
    //     }
    //     Err(err) => {
    //        if err.kind() == ErrorKind::PermissionDenied {
    //             Ok(None)
    //        } else {
    //             Err(err)
    //        }
           
    //     }
        
    //} 
}


fn test_recur(args: &PathBuf){
    if fs::read_dir(&args).is_ok() {
        for files in fs::read_dir(&args).expect("msg"){
            let meta_data = files.as_ref().unwrap().metadata();
            

           let meta_data = files.as_ref().unwrap().metadata().unwrap();
            
            let file = &files.as_ref().unwrap().path();
            match meta_data.is_dir(){
                true =>{
                    let file = &files.unwrap().path();
                    println!("{file:?} is a folder");
                    test_recur(file)
                },
                false => {
                run_command(&meta_data, &file)               
                }
            }
        }
    }
}    

fn read_files(){
    let path = PathBuf::from("/");
    let dir = current_dir().unwrap();
    test_read_files(&path);
    
}

fn run_command(meta_data: &Metadata, file:&PathBuf){
    let size = meta_data.len();
    println!("{:?} has this size: {size}", file)
}