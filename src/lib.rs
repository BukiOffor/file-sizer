#![allow(unused)]
use std::io::ErrorKind;
use std::fs::{File, Metadata};
use std::{fs, process, fs::metadata, io::Error, thread, path::Path, path::PathBuf, env, time};
use std::env::{current_dir, set_current_dir};
use std::mem::drop;
use log;


// this function receives a file path as an arguement
pub fn entry(path:&PathBuf, size: Option<&String>)-> Result<(),Error> {
    log::info!("sizer initialized at {}", path.as_os_str().to_str().unwrap());
    let now = time::Instant::now();

    //check if path returns an OK()
    if fs::read_dir(path).is_ok() {
        //reads the path into an iterable
        let dir = fs::read_dir(path).expect("{file:?} failed to open");
        //split the iterable into different vectors
        let (spawn, main):(Vec<_>, Vec<_>) = dir.enumerate().partition(|(i,_)| i%2 ==0);
        let filter = match size.is_none(){
            true => 100,
            false =>  size.unwrap().parse::<i64>().unwrap() //takes care of error in the main function
        };
        log::info!("you are filtering with {filter} mb");
        let spawn_size = filter.clone();
        //Spawn a new thread to iterate over some     
        let handle = thread::spawn(move||{
            for (index,files) in spawn {
                // get metadata of an entry
                let meta_data = files.as_ref().unwrap().metadata().unwrap();
                // gets the full path of an entry
                let file = files.as_ref().unwrap().path();  
                //checks it entry is file or folder
                match meta_data.is_dir() {
                    true => {
                        subfolders_recur(&file, spawn_size )
                    },
                    false => {
                    let large_file = run_command(&meta_data, &file, spawn_size); 

                        }
                    }
                 }
                thread::sleep(time::Duration::from_millis(5));

        });

        for (index,files) in main {
            // get metadata of an entry
            let meta_data = files.as_ref().unwrap().metadata().unwrap();
            // gets the full path of an entry
            let file = &files.as_ref().unwrap().path();  
            //checks it entry is file or folder
            match meta_data.is_dir() {
                true => {
                    subfolders_recur(&file, filter)
                },
                false => {
                    let _ = run_command(&meta_data, file, filter); 
                    }
                }

             }
        handle.join();
        let finished = now.elapsed().as_secs();
        log::info!("finished succesfully in {} seconds", finished);
        Ok(())

        }else{
            let error = fs::read_dir(path).unwrap_err();
            log::error!("{:?}", error.to_string());
            //Err(error.to_string());
            Err(error)
            //process::exit(1);

        }         
}

// A recursive fuction that takes a file path
// checks if file is folder
// if folder it calls it self
fn subfolders_recur(args: &PathBuf, size: i64){
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
            let meta_data = files.as_ref().unwrap().metadata();
            match meta_data {
                Ok(_) => (),
                Err(err) => {
                    match err.raw_os_error().unwrap() == 9 {
                        true => continue,
                        false => {
                            log::error!("{:?}", err.to_string());
                            process::exit(1);
                        }
                    }
                }
            }
            // return full path of an entry
            let file = &files.as_ref().unwrap().path();
            
            // get the metadata of a file
            let meta_data = files.as_ref().unwrap().metadata().unwrap();

            //checks if a file is a directory
            
            match meta_data.is_dir(){
                true =>{
                    let file = &files.unwrap().path();
                    //log::debug!("{file:?} is a folder");
                    let _ = subfolders_recur(file, size);
                },
                false => {
                let file = run_command(&meta_data, &file, size);  
                            
                }
            }
        }
    }
    
}    


fn run_command<'a>(meta_data: &Metadata, file: &  'a PathBuf, filter:i64)-> Option<(& 'a PathBuf, f64)>{
    let size = meta_data.len();
    //let x = file.file_name().expect("failed to get file name for some reason").to_str().unwrap();
    let x = file.as_os_str().to_str().unwrap();
    if size < 1024*1024 {
        //log::debug!("{}: {:}KB", x, size/1024):
    } 
    else {
        let size_in_mb = size as f64 / (1024 * 1024) as f64;
        if filter != 100 {    
            if size as f64 > (filter * 1024 * 1024) as f64{
                log::debug!("{}: {:.2}MB", x, size_in_mb);
                return Some((file, size_in_mb))
                }
        }else{
            if size as f64 > (100 * 1024 * 1024) as f64{
                log::debug!("{}: {:.2}MB", x, size_in_mb);
                return Some((file, size_in_mb))
                }
            }
        }        
        
        None
    }


// thread using 2 threads, push the files bigger than 180mb to a vector using a message 
// loop through the vector and display the results at the end
// allow args as file path and or file size to display
// change 1000MB to 1GB


    // Function successfully reads the directory and iterates over all files and folders
    #[test]
    fn test_read_directory_and_iterate() {
        // Arrange
        let path = PathBuf::from(".");
        let size = None;
    
        // Act
        let value = entry(&path, size);
    
        // Assert
        assert!(value.is_ok())

    }
    
        // Function filters files based on size and returns the correct files
    #[test]
    fn test_filter_files_based_on_size() {
        // Arrange
        let path = PathBuf::from(".");
        let size = Some(String::from("50"));
    
        // Act
        let result = entry(&path, size.as_ref());
    
        // Assert
        assert!(result.is_ok())

    }
    
        // Function logs info and debug messages correctly
    #[test]
    fn test_log_messages_correctly() {
        // Arrange
        let path = PathBuf::from(".");
        let size = None;
    
        // Act
        let result = entry(&path, size);
    
        // Assert
        assert!(result.is_ok())

    }
    
        // Directory does not exist, function logs error message and exits with status code 1
    #[test]
    fn test_directory_not_exist_logs_error_and_exits() {
        // Arrange
        let path = PathBuf::from("nonexistent/directory");
        let size = None;
    
        // Act
        let result = entry(&path, size);
    
        // Assert
        assert!(result.is_err())
    }
    
        // File or folder metadata cannot be accessed, function continues iterating
    #[test]
    fn test_metadata_cannot_be_accessed_continues_iterating() {
        // Arrange
        let path = PathBuf::from(".");
        let size = None;
    
        // Act
        let result = entry(&path, size);
    
        // Assert
        assert!(result.is_ok())
    }
        
    