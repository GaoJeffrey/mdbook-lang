use std::fs::{self, File};
use std::io::Write;
use std::vec::Vec;
pub use crate::common::{prepare, exe_command, remove_dir_from_error};


pub fn build_cpp(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    let (dir, source_file, output_file) = prepare(
            std::env::temp_dir().to_str().unwrap().to_string(),
            "input.cpp".to_string(), 
            "output.exe".to_string());
    let mut source:File;
    let source_tmp = File::create_new(source_file.clone());
    match source_tmp{
        Ok(file) => source = file,
        Err(e) =>{
            let info = format!("create source file error for {:?}\n detail error: {:?}\n", source_file.clone(), e);
            log::info!("{}", info);
            return info;
        }
    }

    // write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    let result = exe_command("clang++".to_string(), vec![
                                    source_file.as_path().to_str().unwrap().to_string(),
                                    "-o".to_string(),
                                    output_file.as_path().to_str().unwrap().to_string(),
                                ]);
    let error = result.find("error");
        
    match error {
        Some(_) => {
            let _r: Result<(), std::io::Error> = fs::remove_dir_all(dir.clone().as_path());
            let file_name = source_file.as_path().file_name().unwrap().to_str().unwrap().to_string();
            let result = remove_dir_from_error(&result, file_name);
            result
        },
        None => {

            if let Some(_) = result.find("cannot find command"){
                let _r = fs::remove_dir_all(dir.clone().as_path());
                return result
            }

            let mut warning = String::new();
            let file_name = source_file.as_path().file_name().unwrap().to_str().unwrap().to_string();
            let result = remove_dir_from_error(&result, file_name);
            
            if result.len() > 0 {
                warning.push_str(&result);
            }
            let mut sandbox_args_vec = sandbox_args_vec.clone();    
            sandbox_args_vec.push(output_file.as_path().to_str().unwrap().to_string());
        
            let cmd = sandbox_args_vec[0].clone();
            let result = exe_command(cmd, sandbox_args_vec[1..].to_vec());

            let _r = fs::remove_dir_all(dir.clone().as_path());

            if warning.len()> 0 {
                warning.push_str("\n");
                warning.push_str(result.as_str());
                warning
            }else{
                result
            }
        }
    }
}


pub fn build_c(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    build_cpp(code_block, sandbox_args_vec)
}


#[test]
fn build_cpp_test(){
    // load the sandbox configuration
    let sandbox_cmd = std::env::var("MDBOOKLANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOKLANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block=r#"
        #include<iostream>
        using namespace std;
        int main(int argc, char** argv){
            int  i = 1;
            i++;
            cout << "i = " <<i << endl;
        }"#;
    let result = build_cpp(code_block.to_string(),sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("i = 2\n"));
}
#[cfg(test)]
mod mytest{
    use super::*;
#[test]
fn build_c_test(){
    let sandbox_cmd = std::env::var("MDBOOKLANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOKLANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block="
        #include<stdio.h>
        
        int main(int argc, char** argv){
            int i = 1;
            i++;
            printf(\"i = %d\\n\",i);
        }
    ";
    println!("{}", code_block);
    let result = build_c(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("i = 2\n"));
}


#[test]
fn build_cpp_sec_test(){
    // load the sandbox configuration
    let sandbox_cmd = std::env::var("MDBOOKLANG_SERVER_SANDBOX_CMD");
    let mut sandbox_args_vec:Vec<String> = vec![];
    if let Ok(cmd) = sandbox_cmd{
        log::info!("Using sandbox command: {}", cmd);
        sandbox_args_vec.push(cmd);
        let sandbox_args = std::env::var("MDBOOKLANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block=r#"
        #include<iostream>
        #include <dirent.h>

        int main() {
            DIR *dir;
            struct dirent *entry;
            dir = opendir("/"); // 请替换为您要扫描的文件夹路径

            if (dir != NULL) {
                while ((entry = readdir(dir)) != NULL) {
                    std::cout<< entry->d_name<< std::endl;
                }
                closedir(dir);
            }
            return 0;
        }
        "#;
    let result = build_cpp(code_block.to_string(),sandbox_args_vec);
    println!("{}", result);

    assert!(result.len()>0);
}
}