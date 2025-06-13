use clap::{crate_version, Arg, Command};
use mdbook::preprocess::Preprocessor;
use std::process;

use crate::lang::{handle_install};
use clap::ArgMatches;

pub const RESTFUL_API_URL:&str = "http://127.0.0.1:3333/api/v1/build-code";
pub const SERVER_ADDR:&str = "127.0.0.1";
pub const SERVER_PORT:&str = "3333";

use chrono::Local;
use std::io::Write;

#[cfg(target_os = "windows")]
pub const SERVICE_NAME:&str = "mdbook-lang";

#[allow(unused)]
pub struct Cli {
    pub cmd: Command,
}

impl Cli {
    #[cfg(not(target_os="windows"))]
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

    #[cfg(target_os="windows")]
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
            .subcommand(Command::new("install")
                .arg(
                    Arg::new("hostname")
                    .long("hostname")
                    .short('n')
                    .default_value(SERVER_ADDR)
                    .help("Give the `hostname` of programming language compiler server.")
                )
                .arg(
                    Arg::new("port")
                    .long("port")
                    .short('p')
                    .default_value(SERVER_PORT)
                    .help("Give the `port` of programming language compiler server port.")
                )
                .about("Install the programming language compiler server as a windows service.")
            )
            .subcommand(Command::new("uninstall")
                .about("Uninstall the windows service for the programming language compiler server.")
            )
            .subcommand(Command::new("start")
                .about("Start the service for programming language compiler server.")
            )
            .subcommand(Command::new("serve")
                .arg(
                    Arg::new("hostname")
                    .long("hostname")
                    .short('n')
                    .default_value(SERVER_ADDR)
                    .help("Give the `hostname` of programming language compiler server.")
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
            self.log_init();
            handle_install(sub_args);
            process::exit(0);
        }
    }
    #[cfg(target_os = "windows")]
    #[allow(unused)]
    pub fn lang_server(&self, _pre: &dyn Preprocessor) {
        let matches = self.cmd.clone().get_matches();
        if let Some(sub_args) = matches.subcommand_matches("server") {
            self.handle_server(sub_args);
            process::exit(0);
        }
    }

    #[cfg(not(target_os = "windows"))]
    #[allow(unused)]
    pub fn lang_server(&self, _pre: &dyn Preprocessor) {
        
        let matches = self.cmd.clone().get_matches();
        if let Some(sub_args) = matches.subcommand_matches("server") {
            self.log_init();
            self.handle_server(sub_args);
            process::exit(0);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn handle_server(&self, sub_args: &ArgMatches){
        win::handle_server(sub_args);
    }

    #[cfg(not(target_os = "windows"))]
    pub fn handle_server(&self, sub_args: &ArgMatches){
        nix::handle_server(sub_args);
    }
    
    fn log_init(&self){
        let level = std::env::var("MDBOOKLANG_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        env_logger::Builder::from_default_env()
                .format(|buf, record| {
                    writeln!(buf,
                        "{} [{}] ({}): {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.module_path().unwrap(),
                        record.args()
                    )
                })
                .parse_filters(&level)
                .try_init().expect("cannot init env_logger.");
    }
}

#[cfg(not(target_os = "windows"))]
mod nix{
    use clap::ArgMatches;
    use crate::lang;
    use std::process;
    use std::path::PathBuf;
    use std::fs;
    use std::process::Stdio;
    use std::io::Read;
    
    pub fn handle_server(sub_args: &ArgMatches){
        let server_subcommand = sub_args
            .subcommand();
        let hostname:&str;
        let port:&str;
        match server_subcommand{
            Some(("start", ip_addr)) =>{
                log::info!("Start a programming language compiler server.");
                hostname = ip_addr.get_one::<String>("hostname").expect("Should provide ip address, such as `127.0.0.1`.");
                port = ip_addr.get_one::<String>("port").expect("Should provide port, such as `3333`");
                match  find(){
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
                stop();
                process::exit(0);
            }
            Some(("restart", _)) =>{
                log::info!("Restart the programming language compiler server.\n");
                if let Some((ip, port, finded)) = find(){
                    if finded{
                        stop();
                        lang::start_lang_server(ip.as_str(), port.as_str());
                    }else{
                        log::info!("No programming language compiler server running. Cannot restart.")
                    }
                }
                process::exit(0);
            }
            Some(("status", _)) =>{
                log::info!("Check the status of programming language compiler server.\n");
                status();
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

    fn stop(){
        match find(){
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

    fn status(){
        match find(){
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

    fn find() -> Option<(String, String, bool)>{
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
                   
                    err_path.push("mdbook-lang-server.log");
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

#[cfg(target_os = "windows")]
pub mod win{
    use super::*;
    use crate::lang;
    use std::vec;
    pub fn handle_server(sub_args: &ArgMatches){
        let server_subcommand = sub_args
            .subcommand();
        let hostname:&str;
        let port:&str;

        match server_subcommand{
            Some(("install", ip_addr)) =>{
                println!("Install the programming language compiler server as a windows service.");
                hostname = ip_addr.get_one::<String>("hostname").expect("Should provide ip address, such as `127.0.0.1`.");
                port = ip_addr.get_one::<String>("port").expect("Should provide port, such as `3333`");
                
                install(hostname, port);
                process::exit(0);
            }
            Some(("uninstall", _)) =>{
                println!("Uninstall the service for programming language compiler server.");
                uninstall();
                process::exit(0);
            }
            // don't call by user
            Some(("serve", _ip_addr)) =>{
                println!("windows service subsystem calling.\n");
                lang::start_lang_server();
                process::exit(0);
            }
            // don't call by user
            Some(("start", _ip_addr)) =>{
                println!("start service.\n");
                start();
                process::exit(0);
            }
            Some(("stop", _)) => {
                println!("Stop service\n");
                stop();
                process::exit(0);
            }
            Some(("restart", _)) =>{
                println!("Restart service.\n");
                restart();
                process::exit(0);
            }
            Some(("status", _)) =>{
                println!("Check the status of service.\n");
                status();
                process::exit(0);
            }
            Some((subcmd_str, _)) =>{
                println!("Not recognized subcommand `service` subcommand: {}\n", subcmd_str);
                process::exit(-1);
            }
            None =>{
                println!("Please provide a subcommand for the service command, such as install/uninstall/start/stop/restart/status");
                process::exit(-1);
            }
        }

    }

    use std::{ffi::OsString};
    use windows_service::{
        service::{
            ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };
    

    use windows_sys::Win32::Foundation::ERROR_SERVICE_DOES_NOT_EXIST;


    // installs the mdbook-lang service defined in `src/lang.rs#run_service for windows system`.
    fn install(hostname: &str, port: &str){
        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
        let service = service_manager.open_service(SERVICE_NAME, service_access);

        // check the service is not installed before
        if let Ok(_) = service{
            println!("Service {} exists, please uninstall now.", SERVICE_NAME);
            return;
        }

        let service_binary_path = std::env::current_exe()
            .unwrap()
            .with_file_name("mdbook-lang.exe");

        let service_info = ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from(format!("{} playground", SERVICE_NAME)),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::OnDemand,
            error_control: ServiceErrorControl::Normal,
            executable_path: service_binary_path,
            launch_arguments: vec![OsString::from("server"), 
                                   OsString::from("serve"),
                                   OsString::from("-n"),
                                   OsString::from(hostname),
                                   OsString::from("-p"),
                                   OsString::from(port),
                                  ],
            dependencies: vec![],
            account_name: None, // run as System
            account_password: None,
        };
        let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG).unwrap();
        service.set_description("mdbook-lang playground compiler/run server").unwrap();
    }

    fn uninstall()->Option<()>{

        let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
        let service = service_manager.open_service(SERVICE_NAME, service_access);

        // check the service is not installed before
        if let Err(_) = service{
            println!("Service {} doesn't exists.", SERVICE_NAME);
            return None;
        }

        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
        let service = service_manager.open_service(SERVICE_NAME, service_access).unwrap();

        // The service will be marked for deletion as long as this function call succeeds.
        // However, it will not be deleted from the database until it is stopped and all open handles to it are closed.
        service.delete().unwrap();
        // Our handle to it is not closed yet. So we can still query it.
        if service.query_status().unwrap().current_state != ServiceState::Stopped {
            // If the service cannot be stopped, it will be deleted when the system restarts.
            service.stop().unwrap();
        }
        // Explicitly close our open handle to the service. This is automatically called when `service` goes out of scope.
        drop(service);

        // Win32 API does not give us a way to wait for service deletion.
        // To check if the service is deleted from the database, we have to poll it ourselves.
        let start = Instant::now();
        let timeout = Duration::from_secs(5);
        while start.elapsed() < timeout {
            if let Err(windows_service::Error::Winapi(e)) =
                service_manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS)
            {
                if e.raw_os_error() == Some(ERROR_SERVICE_DOES_NOT_EXIST as i32) {
                    println!("{} is deleted.", SERVICE_NAME);
                    return Some(());
                }
            }
            sleep(Duration::from_secs(1));
        }
        println!("{} is marked for deletion..", SERVICE_NAME);
        return None;
    }

    #[test]
    fn test_service_conf(){
        let result = extract_host_port().unwrap();
        println!("{:?}", result);
    }
    pub fn extract_host_port()->Option<(String, String)>{
        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let service = service_manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_CONFIG).unwrap();

        let config = service.query_config().unwrap();
        let v:Vec<&str> = config.executable_path.to_str().unwrap().split(' ').collect();
        
        let mut cli = Cli::new();
        
        let r = cli.cmd.try_get_matches_from_mut(v);
        match r{
            Ok(arg_matcher) =>{
                let se = arg_matcher.subcommand_matches("server").unwrap();
                match  se.subcommand(){
                    Some(sub_args)=>{
                        match sub_args{
                            ("serve", ip_addr) =>{
                                let hostname = ip_addr.get_one::<String>("hostname").expect("Should provide ip address, such as `127.0.0.1`.");
                                let port = ip_addr.get_one::<String>("port").expect("Should provide port, such as `3333`");
                                Some((hostname.to_string(), port.to_string()))
                            }
                            _ =>{
                                None
                            }
                        }
                    }
                    None=>{None}
                }
            }
            Err(e) =>{
                println!("{}", e);
                None
            }
        }
    }
    fn start(){
        let service_name = SERVICE_NAME;

        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let status = service_manager.open_service(
            &service_name,
            ServiceAccess::START | ServiceAccess::QUERY_STATUS,
        );
        match status {
            Ok(service)=>{
                if service.query_status().unwrap().current_state == ServiceState::Running{
                    println!("Service {} is already running", SERVICE_NAME);
                    return;
                }
                // retrive the argument passed when install
                let (hostname, port) = extract_host_port().unwrap();
                service.start(&[hostname, port]).unwrap();
                println!("Service {} is running.", SERVICE_NAME);
            }
            Err(_) =>{
                println!("Service {} doesn't exist. Install it first.", SERVICE_NAME);
            }
        }
        
    }

    fn stop(){
        let service_name = SERVICE_NAME;

        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let status = service_manager.open_service(
            &service_name,
            ServiceAccess::STOP | ServiceAccess::QUERY_STATUS,
        );
        match status{
            Ok(service) =>{
                if service.query_status().unwrap().current_state == ServiceState::Stopped{
                println!("Service {} is already stopped", SERVICE_NAME);
                return;
                }
                service.stop().unwrap();
                println!("Service {} is stopped.", SERVICE_NAME);
            }
            Err(_) =>{
                println!("Service {} doesn't exist.", SERVICE_NAME);
            }
        }
        
    }

    fn restart(){
        let service_name = SERVICE_NAME;

        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let status = service_manager.open_service(
            &service_name,
            ServiceAccess::START | ServiceAccess::STOP | ServiceAccess::QUERY_STATUS,
        );
        match status{
            Ok(service) =>{
                if service.query_status().unwrap().current_state == ServiceState::Running{
                    let mut status = service.stop().unwrap();
                    let start = Instant::now();
                    let timeout = Duration::from_secs(5);
                    while status.current_state !=ServiceState::Stopped && start.elapsed() < timeout {
                        sleep(Duration::from_secs(1));
                        status = service.query_status().unwrap();
                    }
                     println!("Service {} is stopped.", SERVICE_NAME);
                }else{
                    println!("Service {}  is not running. Start it directly.", SERVICE_NAME);
                }
                
                service.start(&vec!["restart service from mdbook-lang"]).unwrap();
                println!("Service {} is running.", SERVICE_NAME);
            }
            Err(_) =>{
                 println!("Service {} doesn't exist.", SERVICE_NAME);
            }
        }
    }
    
    fn status(){
        let manager_access = ServiceManagerAccess::CONNECT;
        let service_manager = ServiceManager::local_computer(None::<&str>, manager_access).unwrap();

        let status = service_manager.open_service(
            SERVICE_NAME,
            ServiceAccess::QUERY_STATUS,
        );
        if let Ok(service) = status{
            let status = service.query_status().unwrap();
       
            match status.current_state{
                    ServiceState::Running => println!("Service {} is running on pid {}.", SERVICE_NAME, status.process_id.unwrap()),
                    ServiceState::Stopped => println!("Service {} is stopped.", SERVICE_NAME),
                    ServiceState::StartPending => println!("Service {} is start pending", SERVICE_NAME),
                    ServiceState::StopPending => println!("Service {} is stop pending", SERVICE_NAME),
                    ServiceState::Paused => println!("Service {} is paused", SERVICE_NAME),
                    ServiceState::ContinuePending => println!("Service {} is continue pending", SERVICE_NAME),
                    ServiceState::PausePending => println!("Service {} is pause pending", SERVICE_NAME),
                }
        }else {
            println!("Service {} isn't exists.", SERVICE_NAME)
        }
        
    }
}