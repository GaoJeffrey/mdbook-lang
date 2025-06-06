use std::io::BufRead;
use std::path::PathBuf;
use std::{fs::DirBuilder, process::Command};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io::{BufReader};
use std::process::Stdio;

#[derive(Serialize,Deserialize, Debug)]
pub struct Code {
    pub lang: String,  
    pub code_block: String,
}

pub fn prepare(dir: String, source_file: String, output_file: String) -> (PathBuf, PathBuf, PathBuf){
    // prepare the temporary file
    let mut path_buf = PathBuf::new();
    path_buf.push(dir);
    let uuid = Uuid::new_v4();
    path_buf.push(uuid.to_string());

    let dir = path_buf.clone();
    let mut source_file_path = dir.clone();
    let mut output_file_path = dir.clone();
    source_file_path.push(source_file);
    output_file_path.push(output_file);

    DirBuilder::new().recursive(true).create(dir.clone()).unwrap();

    (dir, source_file_path, output_file_path)
}


pub fn exe_command(cmd: String, args: Vec<String>)-> String{
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();

    
    let mut result = "".to_string();
    let now = Instant::now();
    let mut cont = true;
    while cont {
        match child.try_wait(){
            Ok(None) => {
                let seconds = now.elapsed().as_secs();
                if seconds > 10{
                    let _ = child.kill();
                    let _ = child.wait();
                    result.push_str("running two long to be killed!");
                    return result;
                }
                thread::sleep(Duration::from_millis(1));
            }
            _ => cont = false,
        }
    }
   
    match child.try_wait(){
        Ok(Some(_)) =>{
            let stdout = child.stdout;
            let stderr = child.stderr;
        //    result.push_str(status.to_string().as_str());
            let out_reader = BufReader::new(stdout.unwrap());
            
            out_reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| result.push_str(format!("{}\n",line).as_str()));

            let err_reader = BufReader::new(stderr.unwrap());
            
            err_reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| result.push_str(format!("{}\n",line).as_str()));
        }
        Ok(None) =>{
            // let r = child.kill().unwrap_err();
            // result.push_str(format!("running two long to be killed! {}", r.to_string()).as_str());
        }
        Err(e) =>{
            log::info!("wait child process error: {}", e.to_string());
        }
    }
    
    result
}

pub fn remove_dir_from_error(result: &String, source_file_name: String) -> String{
    let mut r :String = "".to_string();
    for line in result.lines(){
        if line.contains(source_file_name.as_str()){
            let pos = line.find(source_file_name.as_str()).unwrap();
            r.push_str("/playground/");
            r.push_str(&line[pos..line.len()]);
        }else{
            r.push_str(line);
        }
        r.push_str("\n");
    }
    r

}