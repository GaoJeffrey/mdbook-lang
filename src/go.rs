use std::vec::Vec;

use std::fs::{self, File};
use std::io::Write;

pub use crate::common::{prepare, exe_command, remove_dir_from_error};

pub fn build_go(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    
    let (dir, source_file, output_file) = prepare(
        //"/tmp/mdbook-lang".to_string(), 
        std::env::temp_dir().to_str().unwrap().to_string(),
        "input.go".to_string(), 
        "output.exe".to_string());

    let mut source = File::create_new(source_file.clone()).unwrap();

    // write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    let result = exe_command("go".to_string(), 
                                    [
                                        "build".to_string(),
                                        "-o".to_string(), 
                                        output_file.as_path().to_str().unwrap().to_string(),
                                        source_file.as_path().to_str().unwrap().to_string()].to_vec());
    let result = remove_dir_from_error(&result, "input.go".to_string());

    // if there is no compile error, and execute it and write the result into client
    let error = result.find("command-line-arguments");

    match error {
        Some(_) => {
            let _r = fs::remove_dir_all(dir.clone().as_path());
            result
        },
        None => {
            let mut warning = String::new();

            if result.len() > 0 {
                warning.push_str(&result);
            }
            let mut sandbox_args_vec = sandbox_args_vec.clone();    
            sandbox_args_vec.push(output_file.as_path().to_str().unwrap().to_string());
        
            let cmd = sandbox_args_vec[0].clone();
            let result = exe_command(cmd, sandbox_args_vec[1..].to_vec());

            // let result: String = exe_command(output_file.as_path().to_str().unwrap().to_string(), vec![]);

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



#[test]
fn build_go_test(){
    let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block="package main\nimport \"fmt\"\nfunc main(){\n  i := 1\n  i++\n  fmt.Println(\"i =\", i)\n}";
    let result = build_go(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("i = 2\n"));
}