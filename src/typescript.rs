use std::vec::Vec;

use std::fs::{self, File};
use std::io::Write;

pub use crate::common::{prepare, exe_command, remove_dir_from_error};

pub fn build_typescript(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    let (dir, source_file, output_file) = prepare(
            //"/tmp/mdbook-lang".to_string(), 
            std::env::temp_dir().to_str().unwrap().to_string(),
            "input.ts".to_string(), 
            "input.js".to_string());
    
    let mut source = File::create_new(source_file.clone()).unwrap();

    

    // write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    let mut tsconfig_json_path = dir.clone();
    tsconfig_json_path.push("tsconfig.json");
    let tsconfig_json = r#"
{
        "compilerOptions": {
        "target": "es5", // Or any target
        "lib": ["es2015", "dom"]
        }
}
"#;
    let mut tsconfig_json_file = File::create(tsconfig_json_path.clone()).unwrap();
    let _r = tsconfig_json_file.write_all(tsconfig_json.as_bytes());
    let _r = tsconfig_json_file.flush();
    

    let result = exe_command("tsc".to_string(), 
                                    [
                                        // source_file.as_path().to_str().unwrap().to_string()
                                        "-p".to_string(),
                                        tsconfig_json_path.as_path().to_str().unwrap().to_string(),
                                        ].to_vec());
    let result = remove_dir_from_error(&result, "input.ts".to_string());
    // if there is no compile error, and execute it and write the result into client
    let error = result.find("error");

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
            sandbox_args_vec.push("node".to_string());
            sandbox_args_vec.push(output_file.as_path().to_str().unwrap().to_string());

            let cmd = sandbox_args_vec[0].clone();
            let result = exe_command(cmd, sandbox_args_vec[1..].to_vec());
            let result = remove_dir_from_error(&result, "input.js".to_string());
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
fn build_typescript_test_simple(){
    let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block="console.log('Hello TypeScript')";
    let result = build_typescript(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("Hello TypeScript\n"));
}


#[test]
fn build_typescript_test_complex(){
    let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
    let mut sandbox_args_vec:Vec<String> = vec![];
    if !sandbox_cmd.is_empty() {
        log::info!("Using sandbox command: {}", sandbox_cmd);
        sandbox_args_vec.push(sandbox_cmd);
        let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    let code_block=r#"
class Person {
  name: string;
  age: number;
  occupation?: string;

  constructor(name: string, age: number, occupation?: string) {
    this.name = name;
    this.age = age;
    this.occupation = occupation;
  }

  greet(): string {
    return `Hello, my name is ${this.name} and I am ${this.age} years old.`;
  }
}

const person3 = new Person("Peter Pan", 20, "Adventurer");
console.log(person3.greet()); // Output: Hello, my name is Peter Pan and I am 20 years old.
    "#;
    let result = build_typescript(code_block.to_string(), sandbox_args_vec);
    println!("{}", result);

    assert!(result.eq("Hello, my name is Peter Pan and I am 20 years old.\n"));
}