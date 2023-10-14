#![allow(unused)]
use std::io::ErrorKind;
use std::process::Command;
use std::ffi::OsString;
use std::fs::{File, Metadata};
use std::{fs, fs::metadata, thread, path::Path, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use std::mem::drop;



//use std::path;
fn main() {
    read_files_in_folder();
}

fn read_files_in_folder(){

    let path = PathBuf::from("/Users/mac");
    let dir = current_dir().unwrap();
    for files in fs::read_dir(&dir).expect("{file:?} failed to open"){
        let meta_data = files.as_ref().unwrap().metadata().unwrap();
        let file = &files.as_ref().unwrap().path();  
            match meta_data.is_dir() {
                true => {
                    let path = Path::new(&dir).join(file);
                    println!("{path:?} is about to enter");
                    //let duration = time::Duration::from_millis(100);
                    //thread::sleep(duration);
                    test_recur(&path);
                },
                false => {
                    let result = run_command(&file).unwrap_or_else(|error|{
                        match error.kind(){
                            ErrorKind::PermissionDenied => {
                                println!("permmision to check this file denied");
                            }
                            _ => ()
                        }
                    });
                    
                }

            }
    }
    
}


fn test_recur(args: &PathBuf){
    for files in fs::read_dir(&args).expect("{files:?} failed to open"){
        let meta_data = files.as_ref().unwrap().metadata().unwrap();
        let file = &files.as_ref().unwrap().path();
        match meta_data.is_dir(){
            true =>{
                let file = &files.unwrap().path();
                println!("{file:?}");
                test_recur(file)
            },
            false => {
                run_command(file);
            }
        }
    }
}


fn run_command(args:&PathBuf)->std::io::Result<()>{

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo This is windows"])
            //.spawn()
            .output()
            .expect("failed to execute process")
    } else {
    //thread::sleep(time::Duration::from_millis(100));
    Command::new("du")
        .arg("-hs")
        .arg(args)
        //.spawn()
        .output()?            
        //.expect("failed to execute process")
    };
    println!("{output:?}");
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("{}", s);
    }
    Ok(())


    

}