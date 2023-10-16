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
                    let result = run_command(&file, &meta_data);
                    match result {
                        Ok(Some(_)) => (),
                        Ok(None) => (),
                        Err(e) => panic!("Unexpected error: {}", e),
                    }

            }
    }
    
}


fn test_recur(args: &PathBuf){
    for files in fs::read_dir(&args).expect("msg"){
        let meta_data = files.as_ref().unwrap().metadata().unwrap();
        let file = &files.as_ref().unwrap().path();
        match meta_data.is_dir(){
            true =>{
                let file = &files.unwrap().path();
                println!("{file:?}");
                test_recur(file)
            },
            false => {
               match run_command(file, &meta_data) {
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


fn run_command(args:&PathBuf, metadata:&Metadata)->std::io::Result<Option<()>>{

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

// pub fn impl_thread(path:&PathBuf){
//     log::info!("sizer initialized at {}", path.as_os_str().to_str().unwrap());
    

//     let mut base_folder : Vec<DirEntry>  = Vec::new() ;
//     for files in fs::read_dir(path).expect("{file:?} failed to open"){
//         base_folder.push(files.unwrap());
//     }
//     let folders = base_folder.into_iter();
//     let mut counter = Arc::new(Mutex::new(folders));

//     let main_counter = Arc::clone(&counter);

//     let handle = thread::spawn(move||{
//         let counter = Arc::clone(&counter);

//         loop {
//             log::info!("spawned thread");
//             match counter.lock().unwrap().next() {
//             Some(file) => {
//                 let meta_data = file.metadata().unwrap();
//                 run_command(&meta_data, &file.path())           
//             },
//             None => {
//                 log::info!("sizer ran succesfully");
//                 break
//             }
//         }
//     }
//     });

//     loop {
//         match main_counter.lock().unwrap().next() {
//             Some(file) => {
//                 log::info!("main thread");
//                 let meta_data = file.metadata().unwrap();
//                 run_command(&meta_data, &file.path())            
//             },
//             None => {
//                 log::info!("sizer ran succesfully");
//                 break
//             }
//         }
//     }
//     handle.join();
// }
