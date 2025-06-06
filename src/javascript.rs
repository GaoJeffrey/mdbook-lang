use std::vec::Vec;

use std::fs::{self, File};
use std::io::Write;

pub use crate::common::{prepare, exe_command, remove_dir_from_error};


pub fn build_javascript(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    let (dir, source_file, _output_file) = prepare(
            //"/tmp/mdbook-lang".to_string(), 
            std::env::temp_dir().to_str().unwrap().to_string(),
            "input.js".to_string(), 
            "".to_string());
    
    let mut source = File::create_new(source_file.clone()).unwrap();

    // write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    let mut sandbox_args_vec = sandbox_args_vec.clone();    
    sandbox_args_vec.push("node".to_string());
    sandbox_args_vec.push(source_file.as_path().to_str().unwrap().to_string());

    let cmd = sandbox_args_vec[0].clone();
    let result = exe_command(cmd, sandbox_args_vec[1..].to_vec());

    // let result = exe_command("node".to_string(), 
    //                                 [
    //                                     source_file.as_path().to_str().unwrap().to_string()
    //                                     ].to_vec());
    
    let result = remove_dir_from_error(&result, "input.js".to_string());
    // if there is no compile error, and execute it and write the result into client
    let _r = fs::remove_dir_all(dir.clone().as_path());
    result
}



#[test]
fn build_javascript_test(){
    let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block="console.log('Hello JavaScript')";
    let result = build_javascript(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("Hello JavaScript\n"));
}