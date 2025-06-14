use std::vec::Vec;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use regex::Regex;

pub use crate::common::{prepare, exe_command, remove_dir_from_error};

#[cfg(target_os = "windows")]
pub fn build_java_classpath_separator(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    build_java(code_block, sandbox_args_vec,";")
}

#[cfg(not(target_os = "windows"))]
pub fn build_java_classpath_separator(code_block: String, sandbox_args_vec: Vec<String>) -> String{
    build_java(code_block, sandbox_args_vec,":")
}

#[allow(unused)]
pub fn build_java(code_block: String, sandbox_args_vec: Vec<String>, seperator: &str) -> String{
    let mut java_source_file = "".to_string();
    let mut java_class_file = "".to_string();

    let mut public_class_name = "".to_string();

    let public_class = public_class_or_interface_finder(code_block.as_str());
    match public_class {
        Some(name) => public_class_name = name,
        
        None => public_class_name = "Main".to_string()
    };
    java_source_file = format!("{}.java", public_class_name);

    java_class_file = format!("{}.class", public_class_name);
    

    let (dir, source_file, output_file) = prepare(
            //"/tmp/mdbook-lang".to_string(), 
            std::env::temp_dir().to_str().unwrap().to_string(),
            java_source_file.clone(), 
            java_class_file);
    
    let mut source = File::create_new(source_file.clone()).unwrap();

    // write the source code into file
    let _r = source.write_all(code_block.as_bytes());
    let _r = source.flush();

    let result = exe_command("javac".to_string(), 
                                    [
                                        "-J-Duser.language=en".to_string(),
                                        // "-J-Duser.language=zh".to_string(),
                                        //  "-J-Duser.country=CN".to_string(),
                                        "-encoding".to_string(),
                                        "UTF-8".to_string(),
                                        source_file.as_path().to_str().unwrap().to_string()
                                        ].to_vec());
    let result = remove_dir_from_error(&result, java_source_file);
    // if there is no compile error, and execute it and write the result into client
    let mut  error = result.find("error");
    if let None = error{//how to support multi-language, i.e. German
        error = result.find("错误");
    }

    match error {
        Some(_) => {
            //some error in english or chinese
            let _r = fs::remove_dir_all(dir.clone().as_path());
            result
        },
        None => {
            // no error, find and execute the main class file
            // find warnings if exists
            let mut warning = String::new();

            if result.len() > 0 {
                warning.push_str(&result);
            }
            // let workspace = Path::new(dir.as_path());
            // assert!(std::env::set_current_dir(workspace).is_ok());
            // println!("{:?}", std::env::current_dir());

            // set the classpath to curreng working directory
            let mut classpath = String::new();
            let classpath_old = std::env::var("CLASSPATH");
            if classpath_old.is_ok() {
                classpath.push_str(classpath_old.unwrap().as_str());
            }        
            classpath.push_str(seperator);
            classpath.push_str(dir.to_str().unwrap());
            unsafe {
                std::env::set_var("CLASSPATH".to_string(), classpath);
            }

            // the output_file perhaps is not the main class file
            // check the output files, one or more .class files
            let mut class_files = fs::read_dir(dir.clone().as_path()).unwrap();
            let mut main_class_file:Vec<PathBuf> = vec![];
            let java_find_main = r#"
                    import java.lang.reflect.Method;
                    public class a1fe287f9_6a51_4396_9e97_96ca8a9a83f7{
                        public static void main(String args[]){
                            try {
                                java.net.URL classUrl = new java.io.File(args[0]).toURI().toURL();
                                if(args.length < 1 || classUrl == null){
                                    System.out.println("no_main");
                                    return;
                                }
                                String className = new java.io.File(args[0]).getName().replaceAll("\\.class$", "");
                                ClassLoader loader = new java.net.URLClassLoader(new java.net.URL[]{classUrl});
                                Class<?> myClass = Class.forName(className, false, loader);
                                Method m = myClass.getDeclaredMethod("main", String[].class);
                                if (m == null) {
                                    System.out.println("no_main");
                                }else{
                                    if(m.toString().startsWith("public static void")){
                                        System.out.println("have_main");
                                    }else System.out.println("no_main");
                                }
                            } catch (Exception e) {
                                System.out.println("no_main");
                            }
                        }
                    }
                    "#;
            let mut file_name = PathBuf::new();
            file_name.push(dir.clone().as_path());
            let mut class_file_name = file_name.clone();
            file_name.push("a1fe287f9_6a51_4396_9e97_96ca8a9a83f7.java");
            class_file_name.push("a1fe287f9_6a51_4396_9e97_96ca8a9a83f7");
            let mut file = File::create_new(file_name.clone()).unwrap();
            let _r = file.write_all(java_find_main.as_bytes());
            let _r = file.flush();
            
            let _ = exe_command("javac".to_string(),
                                    [
                                        file_name.clone().as_path().to_str().unwrap().to_string()
                                        ].to_vec());
            
            while let Some(entry) = class_files.next() {
                let entry = entry.unwrap();
                if entry.path().extension().is_some() && 
                    entry.path().extension().unwrap() == "class" 
                    &&entry.path().file_stem().is_some() &&
                    entry.path().file_stem().unwrap() != "a1fe287f9_6a51_4396_9e97_96ca8a9a83f7" {
                    let result = exe_command("java".to_string(), 
                                    [
                                        "a1fe287f9_6a51_4396_9e97_96ca8a9a83f7".to_string(),
                                        entry.path().file_stem().unwrap().to_str().unwrap().to_string()
                                    ].to_vec());
                    if result.contains("have_main") {
                        main_class_file.push(entry.path());
                    }

                }
            }

            if(main_class_file.len() == 0) {
                // no main class found, return error
                let _r = fs::remove_dir_all(dir.clone().as_path());
                return "cannot find main class\n".to_string();
            }
            let mut result:String = String::new();
            if(main_class_file.len() > 1){
                main_class_file.sort();
                result.push_str("more than one main class found, use the first one\n");
                result.push_str("execute the first one class:");
                result.push_str(main_class_file[0].file_stem().unwrap().to_str().unwrap());
                result.push_str("\n");
            }
            let output_file = main_class_file[0].clone();
            log::debug!("find main_class:{:?}", output_file);
            
            let mut sandbox_args_vec = sandbox_args_vec.clone();    
            sandbox_args_vec.push("java".to_string());
            // sandbox_args_vec.push("-J-Duser.language=zh".to_string());
            // "-J-Duser.language=zh".to_string(),
            // "-J-Duser.country=CN".to_string(),
            sandbox_args_vec.push("-Duser.language=zh".to_string());
            sandbox_args_vec.push("-Duser.country=CN".to_string());
            sandbox_args_vec.push(output_file.file_stem().unwrap().to_str().unwrap().to_string());

            log::debug!("command and args are: {:?}", sandbox_args_vec);

            let cmd = sandbox_args_vec[0].clone();
            let r = exe_command(cmd, sandbox_args_vec[1..].to_vec());
            result.push_str(r.as_str());

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

/// check out the first public class/interface/enum type defination, and return the type name otherwise None
pub fn public_class_or_interface_finder(java_code: &str) -> Option<String> {
    // remove single or muti-line comments
    let re_comment = Regex::new(r"//.*|/\*[\s\S]*?\*/").unwrap();
    let code_no_comments = re_comment.replace_all(java_code, "");

    let re = Regex::new(
        r"(?m)public\s+(class|interface|enum)\s+([A-Za-z_][A-Za-z0-9_]*)"
    ).unwrap();

    for cap in re.captures_iter(&code_no_comments) {
        return Some(cap[2].to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use super::*;
    #[test]
    fn test_cn_build(){ 
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)  // 明确设置日志级别
            .try_init()
            .expect("Failed to initialize logger");
        // set the classpath to curreng working directory
        let mut classpath = String::new();
        let classpath_old = std::env::var("CLASSPATH");
        if classpath_old.is_ok() {
            classpath.push_str(classpath_old.unwrap().as_str());
        }        
        classpath.push_str(";");
        classpath.push_str("d:\\rust\\mdbook-lang\\tests");
        unsafe {
            std::env::set_var("CLASSPATH".to_string(), classpath);
        }
        
        let mut cmd = std::process::Command::new("java");
        cmd.arg("HelloWorldCN");

        let child = cmd.stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn();
        match child{
            Ok(mut child)=>{
                log::info!("child is ok");
                child.wait().expect("child is not running");
                        
                let stdout = child.stdout;
                let stderr = child.stderr;
                let mut result = String::new();
                
                let out_reader = std::io::BufReader::new(stdout.unwrap());
                
                out_reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| result.push_str(format!("{}\n",line).as_str()));

                let err_reader = std::io::BufReader::new(stderr.unwrap());
                
                err_reader
                    .lines()
                    .filter_map(|line| line.ok())
                    .for_each(|line| result.push_str(format!("{}\n",line).as_str()));

                log::info!("{}", result);
            }
                
            Err(e)=>{
                log::info!("{}", e);
            }
        }
        


        // assert_eq!(result, "Hello, World!\n");
    }

    #[test]
    fn test_build_one_main_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            public class HelloWorld {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert_eq!(result, "Hello, World!\n");
    }
    #[test]
    fn test_build_one_main_non_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            class HelloWorld {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert_eq!(result, "Hello, World!\n");
    }
    #[test]
    fn test_build_non_main_one_non_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            class HelloWorldNoMain {
                public static void mains(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        println!("result: {}", result);
        assert_eq!(result, "cannot find main class\n");
    }
    #[test]
    fn test_build_non_main_one_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            public class HelloWorldNoMainNoPublic{
                public static void mains(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        println!("result: {}", result);
        assert_eq!(result, "cannot find main class\n");
    }
    #[test]
    fn test_build_one_main_other_parts() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            //public class HelloWorld2{}
            
            class HelloWorld {
                public void fn(){}
                public static int fn2(){return 0;}
                //public void fn3(){}
                public static void main(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert_eq!(result, "Hello, World!\n");
    }
    #[test]
    fn test_build_more_than_one_main_one_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            class HelloWorld1 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
            public class HelloWorld2 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");  // 这个 main 不会被执行
                }
            }  
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert!(result.ends_with("Hello, World!\n"));
    } 
     #[test]
    fn test_build_more_than_one_main_non_public() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            class HelloWorld1 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");
                }
            }
            class HelloWorld2 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!");  // 这个 main 不会被执行
                }
            }  
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert!(result.contains("Hello, World!\n"));
    } 
     #[test]
    fn test_build_two_main_alpha_end() {
        let sandbox_cmd = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_CMD").unwrap_or_else(|_| "".to_string());
        let mut sandbox_args_vec:Vec<String> = vec![];
        if !sandbox_cmd.is_empty() {
            log::info!("Using sandbox command: {}", sandbox_cmd);
            sandbox_args_vec.push(sandbox_cmd);
            let sandbox_args = std::env::var("MDBOOK_LANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
            sandbox_args.split(':')
            .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
        }
        let code = r#"
            class PHelloWorld1 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!1");
                }
            }
            class QHelloWorld2 {
                public static void main(String[] args) {
                    System.out.println("Hello, World!2");  // 这个 main 不会被执行
                }
            }  
        "#;
        let result = build_java_classpath_separator(code.to_string(), sandbox_args_vec);
        assert_eq!(result.contains("Hello, World!1\n"), true);
    } 
    #[test]
    fn test_public_class_basic() {
        let code = r#"
            public class HelloWorld {
                public static void main(String[] args) {}
            }
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("HelloWorld".to_string())
        );
    }

    #[test]
    fn test_public_interface_basic() {
        let code = r#"
            public interface MyInterface {
                void doSomething();
            }
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("MyInterface".to_string())
        );
    }

    #[test]
    fn test_public_enum_ignored() {
        let code = r#"
            enum MyEnum { A, B, C }
            public class AfterEnum {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("AfterEnum".to_string())
        );
    }

    #[test]
    fn test_first_public_class_selected() {
        let code = r#"
            public class First {}
            public class Second {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("First".to_string())
        );
    }

    #[test]
    fn test_first_public_interface_selected() {
        let code = r#"
            public interface First {}
            public interface Second {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("First".to_string())
        );
    }

    #[test]
    fn test_public_class_after_comment() {
        let code = r#"
            // public class CommentedOut {}
            public class RealOne {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealOne".to_string())
        );
    }

    #[test]
    fn test_public_class_after_multiline_comment() {
        let code = r#"
            /*
            public class CommentedOut {}
            */
            public class RealOne {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealOne".to_string())
        );
    }

    #[test]
    fn test_no_public_class_or_interface() {
        let code = r#"
            class PackagePrivate {}
            interface PackagePrivateInterface {}
        "#;
        assert_eq!(public_class_or_interface_finder(code), None);
    }

    #[test]
    fn test_public_class_with_generics() {
        let code = r#"
            public class GenericClass<T> {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("GenericClass".to_string())
        );
    }

    #[test]
    fn test_public_class_with_annotations() {
        let code = r#"
            @SomeAnnotation
            public class AnnotatedClass {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("AnnotatedClass".to_string())
        );
    }

    #[test]
    fn test_public_class_with_leading_spaces() {
        let code = r#"
                public class IndentedClass {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("IndentedClass".to_string())
        );
    }

    #[test]
    fn test_public_class_with_comments_inside() {
        let code = r#"
            public class RealOne { // some comment
            }
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealOne".to_string())
        );
    }

    #[test]
    fn test_public_class_with_interface_before() {
        let code = r#"
            interface NotPublic {}
            public class RealOne {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealOne".to_string())
        );
    }

    #[test]
    fn test_public_interface_with_class_before() {
        let code = r#"
            class NotPublic {}
            public interface RealInterface {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealInterface".to_string())
        );
    }

    #[test]
    fn test_public_class_with_comment_between_public_and_class() {
        let code = r#"
            public /* comment */ class RealOne {}
        "#;
        assert_eq!(
            public_class_or_interface_finder(code),
            Some("RealOne".to_string())
        );
    }
}