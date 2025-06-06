use clap::{crate_version, Arg, Command};
use mdbook::preprocess::Preprocessor;
use std::{ fs, io::Read, path::PathBuf, process::{self, Stdio}, vec};
use crate::lang::{handle_install};
use clap::ArgMatches;
use crate::lang;
pub const RESTFUL_API_URL:&str = "https://127.0.0.1:3333/api/v1/build-code";
pub const SERVER_ADDR:&str = "127.0.0.1";
pub const SERVER_PORT:&str = "3333";

#[allow(unused)]
pub struct Cli {
    pub cmd: Command,
}

impl Cli {
    #[allow(unused)]
    pub fn new() -> Self {
        let cmd = Command::new("mdbook-lang")
        .version(crate_version!())
        .about("A mdbook preprocessor to add multiple programming languages playground support.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor."),
        )
        .subcommand(
            Command::new("install")
                .arg(
                    Arg::new("dir")
                    .default_value(".")
                    .help("Root directory for the book, should contain the configuration file (`book.toml`)")
                    )
                .about("Install the required assset files and include it in the config."),
        )
        .subcommand(
            Command::new("server")
            .args_conflicts_with_subcommands(true)
            .subcommand(Command::new("start")
                .arg(
                    Arg::new("hostname")
                    .long("hostname")
                    .short('n')
                    .default_value(SERVER_ADDR)
                    .help("Give the `host` of programming language compiler server.")
                )
                .arg(
                    Arg::new("port")
                    .long("port")
                    .short('p')
                    .default_value(SERVER_PORT)
                    .help("Give the `port` of programming language compiler server port.")
                )
                .about("Start the programming language compiler server.")
            )
            .subcommand(Command::new("stop")
                .about("Stop the programming language compiler server.")
            )
            .subcommand(Command::new("restart")
                .about("Reload programming language compiler server.")
            )
            .subcommand(Command::new("status")
                .about("Show the status of programming language compiler server.")
            )
            .about("start/stop/reload the programming language compiler server"),
        );

        Self { cmd }
    }

    #[allow(unused)]
    pub fn lang_supports(&self, pre: &dyn Preprocessor) {
        let matches = self.cmd.clone().get_matches();
        if let Some(sub_args) = matches.subcommand_matches("supports") {
            // get the renderer
            let renderer = sub_args.get_one::<String>("renderer").unwrap();

            // signal whether the renderer is supported by exiting with 1 or 0.
            if pre.supports_renderer(renderer) {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
    }
    #[allow(unused)]
    pub fn lang_install(&self, _pre: &dyn Preprocessor) {
        let matches = self.cmd.clone().get_matches();
        if let Some(sub_args) = matches.subcommand_matches("install") {
            handle_install(sub_args);
            process::exit(0);
        }
    }
    #[allow(unused)]
    pub fn lang_server(&self, _pre: &dyn Preprocessor) {
        let matches = self.cmd.clone().get_matches();
        if let Some(sub_args) = matches.subcommand_matches("server") {
            self.handle_server(sub_args);
            process::exit(0);
        }
       
        
    }


    pub fn handle_server(&self, sub_args: &ArgMatches){
        let server_subcommand = sub_args
            .subcommand();
        let hostname:&str;
        let port:&str;
        match server_subcommand{
            Some(("start", ip_addr)) =>{
                log::info!("Start a programming language compiler server.");
                hostname = ip_addr.get_one::<String>("hostname").expect("Should provide ip address, such as `127.0.0.1`.");
                port = ip_addr.get_one::<String>("port").expect("Should provide port, such as `3333`");
                match  self.find(){
                    Some((n, p, flag)) =>{
                        if flag {
                            log::info!("Another programming language compiler server is running on {}:{}.", n, p)
                        }else{
                            lang::start_lang_server(hostname, port);
                        }
                    }
                    None =>{
                        lang::start_lang_server(hostname, port);
                    }
                }
                process::exit(0);
            }
            Some(("stop", _)) => {
                log::info!("Stop the programming language compiler server.\n");
                self.stop();
                process::exit(0);
            }
            Some(("restart", _)) =>{
                log::info!("Restart the programming language compiler server.\n");
                if let Some((ip, port, finded)) = self.find(){
                    if finded{
                        self.stop();
                        lang::start_lang_server(ip.as_str(), port.as_str());
                    }else{
                        log::info!("No programming language compiler server running. Cannot restart.")
                    }
                }
                process::exit(0);
            }
            Some(("status", _)) =>{
                log::info!("Check the status of programming language compiler server.\n");
                self.status();
                process::exit(0);
            }
            Some((subcmd_str, _)) =>{
                log::info!("Not recognized subcommand `server` subcommand: {}\n", subcmd_str);
                process::exit(-1);
            }
            None =>{
                log::info!("Please provide a subcommand for the server command, such as start/stop/restart/status");
                process::exit(-1);
            }
        }

    }


    fn stop(&self){
        
        match self.find(){
            Some((_ip, _addr, flag)) => {
                if flag{
                    let mut tmp = PathBuf::new();
                    tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
                    let mut pid_path = tmp.clone();
                    pid_path.push("mdbook-lang-server.pid");
                    
                    let pid = fs::read_to_string(pid_path).unwrap();
                    // kill it
                    let pid:Vec<&str> = pid.split("\n").collect();
                    let pid = pid[0];
                    
                    let mut killer = std::process::Command::new("kill");
                    killer.args(vec![pid.to_string()]);
                    let status = killer.status().expect("Terminate mdbook-lang programming language compiler server");
                    log::info!("mdbook-lang programming language compiler server is terminated at the exit code of {status}");
                }else {
                    log::info!("compiler server is not started");
                }
            }
            None =>{
                log::info!("compiler server is not started.");
            }
        }
    }

    fn status(&self){
        match self.find(){
            Some((ip, port, flag)) => {    
                if flag{
                    log::info!("A programming language compiler server is running:listen on {}:{}.", ip, port)
                }else{
                    log::info!("No programming language compiler server has been started.");
                    let mut tmp = PathBuf::new();
                    tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
                    let mut pid_path = tmp.clone();
                    pid_path.push("mdbook-lang-server.pid");

                    let _ = fs::remove_file(pid_path);
                }
            }
            None => {
                log::info!("No programming language compiler server has been started.\n");
            }
        }
    }


    fn find(&self) -> Option<(String, String, bool)>{
        // find pid
        let mut tmp = PathBuf::new();
        tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
        let mut pid_path = tmp.clone();
        pid_path.push("mdbook-lang-server.pid");
        let pid = fs::read_to_string(pid_path);

        match pid{
            Ok(pid) =>{
                let pid:Vec<&str> = pid.split("\n").collect();
                let pid = pid[0];
                let ps = std::process::Command::new("ps")
                    .args(vec![
                        "-ef", 
                    ])
                    .stdout(Stdio::piped()).spawn().unwrap();

                let grep1 = std::process::Command::new("grep")
                    .args(vec![
                        pid,
                    ])
                    .stdin(ps.stdout.unwrap())
                    .stdout(Stdio::piped()).spawn().unwrap();

                let grep2 = std::process::Command::new("grep")
                    .args(vec![
                        "-v",
                        "grep",
                    ])
                    .stdin(grep1.stdout.unwrap())
                    .stdout(Stdio::piped()).spawn().unwrap();

                let awk = std::process::Command::new("awk")
                    .args(vec![

                        "{print $8}", 
                    ])
                    .stdin(grep2.stdout.unwrap())
                .stdout(Stdio::piped()).spawn().unwrap();


                let mut awk = awk.stdout.unwrap();
                let mut mdbook_lang_str:String = String::new();
                
                let size = awk.read_to_string(&mut mdbook_lang_str).unwrap();
                
                if size >0 && mdbook_lang_str.contains("mdbook-lang"){
                    // find ipaddress and port
                    let mut err_path = tmp.clone();
                   
                    err_path.push("mdbook-lang-server.err");
                    if let Ok(ip_addr_port) = fs::read_to_string(err_path){
                        let tmp = ip_addr_port.clone();
                        let result:Vec<&str> = tmp.split("\n").collect();
                        let mut result:Vec<&str> = result[2].split(" ").collect();
                        result.reverse();
                        let ip_port = result[0];
                        let ip_port:Vec<&str> = ip_port.split(":").collect();
                        let ip = ip_port[0];
                        let port:Vec<&str> = ip_port[1].split("\n").collect();
                        let port = port[0];
                        return Some((ip.to_string(), port.to_string(),true));
                    }
                    return Some(("".to_string(), "".to_string(),true));
                    
                }else{
                    log::info!("mdbook-lang's language compiler server is not running ...");
                    let mut tmp = PathBuf::new();
                    tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
                    let mut pid_path = tmp.clone();
                    pid_path.push("mdbook-lang-server.pid");

                    let _ = fs::remove_file(pid_path);
                    return Some(("".to_string(), "".to_string(),false));
                }
            }
            Err(_e) => {
                return None;
            }
        }
    }
}