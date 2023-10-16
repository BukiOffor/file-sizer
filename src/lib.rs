#![allow(unused)]

use std::io::ErrorKind;
use std::fs::{File, Metadata};
use std::{fs, fs::metadata, io::Error, thread, path::Path, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use std::mem::drop;
use log;



// this function receives a file path as an arguement
pub fn test_read_files(path:&PathBuf){

    if fs::read_dir(path).is_ok() {
        for files in fs::read_dir(path).expect("{file:?} failed to open"){
            // get metadata of an entry
            let meta_data = files.as_ref().unwrap().metadata().unwrap();
            // gets the full path of an entry
            let file = &files.as_ref().unwrap().path();  
            //checks it entry is file or folder
            match meta_data.is_dir() {
                true => {
                    test_recur(&file)
                },
                false => {
                    run_command(&meta_data, file) 
                    }
                }

             }
        }else{
            let error = fs::read_dir(path).unwrap_err();
            log::error!("{:?}", error.to_string());
        }         
}

// A recursive fuction that takes a file path
// checks if file is folder
// if folder it calls it self
fn test_recur(args: &PathBuf){
    if fs::read_dir(&args).is_ok() {
        // reads the files in a dir, checked with the is_ok() method above that file is elligible to read
        for files in fs::read_dir(&args).expect("msg"){
            //tries to read the metadata, if metadata returns a permission we skip the file
            let meta_data = files.as_ref().unwrap().metadata();
            match meta_data {
                Ok(data) => (),
                Err(error) => {
                    match error.kind(){
                        ErrorKind::PermissionDenied => continue,
                        err => log::error!("{err:?}")
                    }
                }
            }
            // get the metadata of a file
            let meta_data = files.as_ref().unwrap().metadata().unwrap();
            // return full path of an entry
            let file = &files.as_ref().unwrap().path();
            //checks if a file is a directory
            match meta_data.is_dir(){
                true =>{
                    let file = &files.unwrap().path();
                    //log::debug!("{file:?} is a folder");
                    test_recur(file)
                },
                false => {
                run_command(&meta_data, &file)               
                }
            }
        }
    }
}    


fn run_command(meta_data: &Metadata, file:&PathBuf){
    let size = meta_data.len();
    let x = file.file_name().expect("failed to get file name for some reason").to_str().unwrap();
    
    if size < 1024*1024 {
        //log::debug!("{}: {:}KB", x, size/1024);
    } else{
        let size_in_mb = size as f64 / (1024 * 1024) as f64;
        log::debug!("{:?}: {:.2}MB", x, size_in_mb);
        //if size_in_mb > (1024 * 1024) as f64{
           // log::debug!("{:?}: {}MB", x, size_in_mb);
       // }
    }   
}

// thread using 2 threads, push the files bigger than 180mb to a vector using a message
// loop through the vector and display the results at the end
// allow args as file path and or file size to display
// change 1000MB to 1GB