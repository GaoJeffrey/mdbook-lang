use std::{process};
mod cfg;
mod cli;
mod lang;
use lang::handle_preprocessing;

mod common;
mod cpp;
mod java;
mod python;
mod go;
mod javascript;
mod typescript;
mod scheme;

pub use common::{prepare, exe_command, Code};
pub use cpp::{build_cpp, build_c};
pub use java::build_java_classpath_separator;
pub use python::build_python;
pub use go::build_go;
pub use scheme::build_scheme;
pub use javascript::build_javascript;
pub use typescript::build_typescript;

fn main(){
    let cli = cli::Cli::new();
    let lang = lang::Lang::new();

    // nix: mdbook-lang --server start/stop/restart command line argument
    // win: mdbook-lang --server install/uninstall/start/stop/restart
    cli.lang_server(&lang);
    
    // mdbook-lang --supports command line argument
    cli.lang_supports(&lang);

    // mdbook-lang --install command line argument
    cli.lang_install(&lang);
    
    if let Err(e) = handle_preprocessing(&lang) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
#[cfg(not(target_os = "windows"))]
mod nix{
#[cfg(test)]
mod test{
    use crate::*;
    use lang::handle_install_work;
    use tempfile;
    use std::fs;
    #[test]
    fn test_install() {
        let input = fs::read_to_string("tests/empty.toml".to_string()).expect("can't read file");

        // Create a temporary directory
        let tmp = tempfile::tempdir().expect("can't create tempdir");

        let book_toml = tmp.path().join("book.toml");
        fs::write(&book_toml, input).expect("can't write book.toml");
        handle_install_work(tmp.path().to_path_buf());
    }
    #[test]
    fn test_server_start_with_two_args(){
        let level = std::env::var("MDBOOKLANG_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        env_logger::init_from_env(env_logger::Env::default().default_filter_or(level));
        
        let mut cli = cli::Cli::new();
        let r = cli.cmd.try_get_matches_from_mut(vec!["mdbook-lang", "server", "start", "--hostname", "127.0.0.1", "--port", "3333"]);
        match r{
            Ok(arg_matcher) =>{
                if let Some(sub_args) = arg_matcher.subcommand_matches("server") {
                    cli.handle_server(sub_args);
                    process::exit(0);
                }
            }
            
            Err(e) =>{
                log::info!("{}", e);
            }
        }
    }
    #[test]
    fn test_server_restart(){
        let level = std::env::var("MDBOOKLANG_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        env_logger::init_from_env(env_logger::Env::default().default_filter_or(level));
        
        let mut cli = cli::Cli::new();
        let r = cli.cmd.try_get_matches_from_mut(vec!["mdbook-lang", "server", "restart"]);
        match r{
            Ok(arg_matcher) =>{
                if let Some(sub_args) = arg_matcher.subcommand_matches("server") {
                    cli.handle_server(sub_args);
                    process::exit(0);
                }
            }
            
            Err(e) =>{
                log::info!("{}", e);
            }
        }
    }
}
}

#[cfg(target_os = "windows")]
mod win{
mod test{
    #[allow(unused)]
    use crate::*;
    #[test]
    fn test_service_install(){
        let level = std::env::var("MDBOOKLANG_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        env_logger::init_from_env(env_logger::Env::default().default_filter_or(level));
        
        let mut cli = cli::Cli::new();
        let r = cli.cmd.try_get_matches_from_mut(vec!["mdbook-lang", "server", "install", "--hostname", "127.0.0.1", "--port", "3333"]);
        match r{
            Ok(arg_matcher) =>{
                if let Some(sub_args) = arg_matcher.subcommand_matches("server") {
                    cli.handle_server(sub_args);
                    process::exit(0);
                }
            }
            Err(e) =>{
                log::info!("{}", e);
            }
        }
    }
}
}