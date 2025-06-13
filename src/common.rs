
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

#[cfg(target_os = "windows")]
use std::borrow::Cow;
#[cfg(target_os = "windows")]
use std::path::Path;

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
// #[cfg(not(target_os = "windows"))]
// fn enhance_exe_name(exe_name: &Path) -> Cow<Path> {
//     exe_name.into()
// }

#[cfg(target_os = "windows")]
fn enhance_ext_name<'a>(exe_name: &'a Path, ext: &'a str) -> Cow<'a, Path> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let raw_input: Vec<_> = exe_name.as_os_str().encode_wide().collect();
    let raw_extension: Vec<_> = OsStr::new(ext).encode_wide().collect();

    if raw_input.ends_with(&raw_extension) {
        exe_name.into()
    } else {
        let mut with_exe = exe_name.as_os_str().to_owned();
        with_exe.push(ext);
        PathBuf::from(with_exe).into()
    }
}

#[cfg(target_os = "windows")]
fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}
#[cfg(target_os="windows")]
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_command_rename(){
        let cmd = "clang++".to_string();
        log::debug!("{}", std::env!("PATH"));
        let path= Path::new(cmd.as_str());
        let cmd_exe = enhance_ext_name(&path,".exe");
        log::debug!("cmd_exec: is {:?}", cmd_exe);
        let mut cmd_path: PathBuf = PathBuf::new();

        match find_it(cmd_exe){
            Some(path) => {
                cmd_path = path;
            }
            None =>{
                log::info!("cannot find command :{}\n", cmd);
            }
        }

        assert_eq!(cmd_path, PathBuf::from("D:\\ds-soft\\Emacs-llvmMinGW64-CMake-v1.0.6\\third-party\\llvm-mingw-20230614-ucrt-x86_64\\bin\\clang++.exe"));

        log::debug!("cmd: is {:?}", cmd);
        let mut cmd = Command::new(cmd_path);
        
        let r = cmd.spawn();
        match r{
            Ok(r) =>{
                println!("ok ...");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
        
    }

    #[test]
    fn test_find_it(){
        let cmd = "clang++".to_string();
        // log::debug!("{}", std::env!("PATH"));
        // let path= Path::new(cmd.as_str());
        // let cmd_exe = enhance_ext_name(&path,".exe");
        // log::debug!("cmd_exec: is {:?}", cmd_exe);
        // let mut cmd_path: PathBuf = PathBuf::new();

        // match find_it(cmd_exe){
        //     Some(path) => {
        //         cmd_path = path;
        //     }
        //     None =>{
        //         log::info!("cannot find command :{}\n", cmd);
        //     }
        // }

        let mut cmd_path: PathBuf = PathBuf::new();
        match command_rename(cmd.clone()){
            Some(path) => {
                cmd_path = path;
            }
            None =>{
                log::info!("cannot find command :{}\n", cmd);
            }
        }

        assert_eq!(cmd_path, PathBuf::from("D:\\ds-soft\\Emacs-llvmMinGW64-CMake-v1.0.6\\third-party\\llvm-mingw-20230614-ucrt-x86_64\\bin\\clang++.exe"));

        log::debug!("cmd: is {:?}", cmd);
        let mut cmd = Command::new(cmd_path);
        
        let r = cmd.spawn();
        match r{
            Ok(r) =>{
                println!("ok ...");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
        
    }
}

#[cfg(target_os="windows")]
fn command_rename(cmd_name: String) -> Option<PathBuf>{
    let exts = vec![".exe", ".bat", ".cmd"];
    for item in exts{
        let path= Path::new(cmd_name.as_str());
        let cmd_exe = enhance_ext_name(&path, item);
        log::debug!("cmd_exec: is {:?}", cmd_exe);
        if let Some(path) = find_it(cmd_exe){
            return Some(path)
        }
    }
    None
}

#[cfg(target_os="windows")]
pub fn exe_command(cmd: String, args: Vec<String>)-> String{
    #[allow(unused)]
    let mut cmd_path: PathBuf = PathBuf::new();
    match command_rename(cmd.clone()){
        Some(path) => {
            cmd_path = path;
        }
        None =>{
            log::info!("cannot find command :{}\n", cmd);
            return format!("cannot find command :{}\n", cmd);
        }
    }

    log::debug!("cmd: is {:?}", cmd_path);
    let mut cmd = Command::new(cmd_path);
    
    cmd.args(args);
   
    let child_tmp = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
   
    match child_tmp{
        Ok(mut child) =>{
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
            log::debug!("leaving exe_command ...");
            
            result
        }
        Err(e) => {
            log::debug!("execute command {:?} error: {:?}", cmd, e);
            e.to_string()
        }
    }
}

#[cfg(not (target_os="windows"))]
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