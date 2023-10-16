#![allow(unused)]

use std::{fs, thread, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use sizer::test_read_files;


fn main(){
    env_logger::init();

    // reads the home directory
    let home_dir = PathBuf::from("/Users/mac/Downloads");
    //gets the current directory
    let current_dir = current_dir().unwrap();
    test_read_files(&home_dir);
    //println!("{:?}",home_dir)
    
    
}