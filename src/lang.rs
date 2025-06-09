use std::{env, fs::{self, File}, process, thread};
use std::io::{self, Write};
use std::path::PathBuf;

use clap::ArgMatches;
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
    Config,
};


use log;
use regex::Regex;
use uuid::Uuid;
use toml_edit::{value, Array, DocumentMut, Item, Table, Value};

use crate::{cfg, Code};
use tower_http::cors::{Any, CorsLayer};

use axum::{
    http::{header::CONTENT_TYPE, Method}, routing::{get, post}, Json, Router
};
use daemonize::Daemonize;
use signal_hook::{consts::{SIGINT, SIGQUIT, SIGTERM}, iterator::Signals};

use crate::cli;
use crate::build_c;
use crate::build_cpp;
use crate::build_go;
use crate::build_java;
use crate::build_python;
use crate::build_javascript;
use crate::build_typescript;
use crate::build_scheme;

const LANG_HTML: &[u8] = include_bytes!("assets/lang.html");
const LANG_JS: &[u8] = include_bytes!("assets/lang.js");
const LANG_CSS: &[u8] = include_bytes!("assets/lang.css");
const JQUERY_JS: &[u8] = include_bytes!("assets/jquery.js");
const DISABLE_DEVTOOL_JS: &[u8] = include_bytes!("assets/disable-devtool.js");

const LANG_FILES: &[(&str, &[u8])] = &[
    ("lang.html", LANG_HTML),
    ("lang.js", LANG_JS),   
    ("lang.css", LANG_CSS),  
    ("jquery.js", JQUERY_JS),   
    ("disable-devtool.js", DISABLE_DEVTOOL_JS),
];

pub struct Lang;

impl Lang {
    #[allow(unused)]
    pub fn new() -> Lang {
        Lang
    }
}

fn get_asset(name: &str) -> String {
    for (name_item, content) in LANG_FILES {
        if name == *name_item {
            return String::from_utf8_lossy(content).to_string();
        }
    }
    log::error!("Asset {} not found", name);
    panic!("Asset {} not found", name);
}

fn parse_options(options_str: &str) -> Vec<String> {
    options_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn map_lang(raw_lang: &str) -> &str {
    match raw_lang {
        "lisp" | "scheme" => "scheme",
        "py" | "python" => "python",
        "ts" | "typescript" => "typescript",
        "js" | "javascript" => "javascript",
        "java" => "java",
        "cpp" | "c++" | "c" => "cpp",
        "go" => "go",
        // _ => "cpp",
        _ => raw_lang
    }
}

fn render_langs(content: &str, config: &Config) -> (bool, String) {
    
    // \r? is for windows line endings
    let langs = r"\blisp\b|\bscheme\b|\bcpp\b|\bc++\b|\bc\b|\bjava\b|\bpy\b|\bpython\b|\bts\b|\btypescript\b|\bjs\b|\bjavascript\b|\bgo\b";
    let re: Regex = Regex::new(&format!(r"(?s)```({}),?(.*?)\r?\n(.*?)```", langs)).unwrap();

    // if there are no matches, return the content as is
    if !re.is_match(content) {
        return (false, content.to_string());
    }
   

    // replace all matches with the lang html
    let rendered = re
        .replace_all(content, |caps: &regex::Captures| {
            let uuid = Uuid::new_v4().to_string();
            let code = caps.get(3).map(|m| m.as_str()).unwrap_or("").trim();

            let raw_lang = caps.get(1).map(|m| m.as_str()).unwrap_or("").trim();

            let lang = map_lang(raw_lang);
            let codeblock = format!("```{}\n{}\n```", lang, code);
            let options_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let options = parse_options(options_str);
            let editable = options.contains(&"editable".to_string());
            let editable_config: bool = cfg::get_config_bool(config, "editable", false);
            let mut editable = editable || editable_config;
            
            if options.contains(&"editable=false".to_string())
            {
                editable = false;
            }

            let noplayground = options.contains(&"norun".to_string());
            // get the config options
            let enable = cfg::get_config_bool(config, &format!("{}-enable", lang), false);
            
            
            let mut restful_api_server = cfg::get_config_string(
                config,
                "server",
                "noserver",
            );

            if restful_api_server.contains("noserver"){
                restful_api_server = cfg::get_config_string(
                    config,
                    &format!("{}.server", lang),
                    cli::RESTFUL_API_URL,
                );
            }

            // if lang is in the options, return the code block as is
            if !enable || options.contains(&"nolang".to_string()) {
                return format!("```{}\n{}\n```", lang, code);
            }

            // disable-devtool configuration
            let disable_devtool_auto:bool = cfg::get_config_bool(config, "disable-devtool-auto", false);
            let disable_menu: bool = cfg::get_config_bool(config, "disable-menu", false);
            let clear_log: bool = cfg::get_config_bool(config, "clear-log", false);
            let disable_select: bool = cfg::get_config_bool(config, "disable-select", false);
            let disable_copy: bool = cfg::get_config_bool(config, "disable-copy", false);
            let disable_cut: bool = cfg::get_config_bool(config, "disable-cut", false);
            let disable_paste: bool = cfg::get_config_bool(config, "disable-paste", false);


            // ACE editor strict
            let ace_strict: bool = cfg::get_config_bool(config, "ace-strict", false);
            // 10 lines in vec editor is:  height: 216px
            let lines = (codeblock.lines().count()+3) * 23 ;
            get_asset("lang.html")
                .replace("{lang}", lang)
                .replace("{codeblock}", &codeblock)                
                .replace("{editable}", if editable { "editable" } else {"" })
                .replace("[norun]", if noplayground { "norun" } else { "" })
                .replace("[lines]", &lines.to_string())
                .replace("{uuid}", &uuid)
                .replace("{restfulserver}", &restful_api_server)
                // for disable-devtool options
                .replace("{auto}", &format!("{}",if disable_devtool_auto{ "disable-devtool-auto"} else{""}))
                .replace("{menu}", &format!("{}",disable_menu))
                .replace("{clear}", &format!("{}",clear_log))
                .replace("{select}", &format!("{}",disable_select))
                .replace("{copy}", &format!("{}",disable_copy))
                .replace("{cut}", &format!("{}",disable_cut))
                .replace("{paste}", &format!("{}",disable_paste))
                // fro disable ACE editor copy/cut/past or not
                .replace("{strict}", &format!("{}",ace_strict))
        })
        .to_string();
    (true, rendered)
}

impl Preprocessor for Lang {
    fn name(&self) -> &str {
        "mdbook-lang"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = &ctx.config;

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {

                let (mdbook_lang_found, rendered) = render_langs(&chapter.content, config);
                // log::debug!("goted repl_found: {} \n {}", mdbook_lang_found, rendered);
                if mdbook_lang_found {
                    chapter.content = rendered;
                }
            }
        });
        // log::info!("Preprocessed book:\n{:?}", book);

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

#[allow(unused)]
pub fn handle_preprocessing(lang: &Lang) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-lang preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }
    let processed_book = lang.run(&ctx, book)?;

   
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}



#[allow(unused)]
pub fn handle_install_work(proj_dir: PathBuf){
    let config = proj_dir.join("book.toml");

    if !config.exists() {
        log::error!("Configuration file '{}' missing", config.display());
        process::exit(1);
    }

    log::info!("Reading configuration file {}", config.display());
    let toml = fs::read_to_string(&config).expect("can't read configuration file");
    println!("logging:\n{}", toml);
    
    let mut doc = toml
        .parse::<DocumentMut>()
        .expect("configuration is not valid TOML");

    let has_pre = has_preprocessor(&mut doc);
    if !has_pre {
        log::info!("Adding preprocessor configuration");
        add_preprocessor(&mut doc);
    }
    log::info!("after preprocessor check:\n{}", doc.to_string());
    let added_files = add_additional_files(&mut doc);
    

    if !has_pre || added_files {
        log::info!("Saving changed configuration to {}", config.display());
        
        let toml = doc.to_string();
        log::debug!("before writing doc in add additional files:\n{}", doc.to_string());
        log::debug!("before writing toml in add additional files:\n{}", toml);
        let mut file = File::create(config).expect("can't open configuration file for writing.");
        file.write_all(toml.as_bytes())
            .expect("can't write configuration");
    }

    log::debug!("after add additional files:\n{}", doc.to_string());
    let mut printed = false;
    for (name, content) in LANG_FILES {
        let filepath = proj_dir.join(name);
        if filepath.exists() {
            log::debug!(
                "'{}' already exists (Path: {}). Skipping.",
                name,
                filepath.display()
            );
        } else {
            if !printed {
                printed = true;
                log::info!(
                    "Writing additional files to project directory at {}",
                    proj_dir.display()
                );
            }
            log::debug!("Writing content for '{}' into {}", name, filepath.display());
            let mut file = File::create(filepath).expect("can't open file for writing");
            file.write_all(content)
                .expect("can't write content to file");
        }
    }

    log::info!("Files & configuration for langbook are installed. You can start using it in your book.");
    let codeblock = r#"
```cpp
#include <iostream>

int main(int argc, char *argv[]) {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
```
"#;
    log::info!("Add a code block like:\n{}", codeblock);

    process::exit(0);
}
#[allow(unused)]
pub fn handle_install(sub_args: &ArgMatches){
    let proj_dir = sub_args
        .get_one::<String>("dir")
        .expect("Required argument");
    let proj_dir = PathBuf::from(proj_dir);
    handle_install_work(proj_dir);
}

fn add_additional_files(doc: &mut DocumentMut) -> bool {

    log::debug!("before add_additional_files:\n{}", doc);
    let mut changed = false;
    // let mut printed = false;

    let file = "jquery.js";
    let additional_js = additional(doc, "js");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-js'. Skipping", file)
    } else {
        // printed = true;
        log::info!("Adding additional files to configuration");
        log::debug!("Adding '{}' to 'additional-js'", file);
        insert_additional(doc, "js", file);
        changed = true;
    }

    let file = "disable-devtool.js";
    let additional_js = additional(doc, "js");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-js'. Skipping", file)
    } else {
        // printed = true;
        log::info!("Adding additional files to configuration");
        log::debug!("Adding '{}' to 'additional-js'", file);
        insert_additional(doc, "js", file);
        changed = true;
    }

    let file = "lang.js";
    let additional_js = additional(doc, "js");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-js'. Skipping", file)
    } else {
        // printed = true;
        log::info!("Adding additional files to configuration");
        log::debug!("Adding '{}' to 'additional-js'", file);
        insert_additional(doc, "js", file);
        changed = true;
    }
    
    let file = "lang.css";
    let additional_js = additional(doc, "css");
    if has_file(&additional_js, file) {
        log::debug!("'{}' already in 'additional-css'. Skipping", file)
    } else {
        // printed = true;
        log::info!("Adding additional files to configuration");
        log::debug!("Adding '{}' to 'additional-css'", file);
        insert_additional(doc, "css", file);
        changed = true;
    }

    log::debug!("end add_additional_files:\n{}", doc);
    changed
}

fn additional<'a>(doc: &'a mut DocumentMut, additional_type: &str) -> Option<&'a mut Array> {
    let doc = doc.as_table_mut();

    let item = doc.get_mut("output")?;
    let item = item.as_table_mut()?.get_mut("html")?;
    let item = item
        .as_table_mut()?
        .get_mut(&format!("additional-{}", additional_type))?;
    item.as_array_mut()
}

fn has_preprocessor(doc: &mut DocumentMut) -> bool {
    doc.get("preprocessor")
        .and_then(|p| p.get("lang"))
        .map(|m| matches!(m, Item::Table(_)))
        .unwrap_or(false)
}

fn add_preprocessor(doc: &mut DocumentMut) {
    let doc = doc.as_table_mut();

    let empty_table = Item::Table(Table::default());

    let item = doc.entry("preprocessor").or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("lang")
        .or_insert(empty_table);
    item["command"] = value("mdbook-lang");
    // compiling server address
    item["server"] = value("http://127.0.0.1:3333/api/v1/build-code");
    // programming language code block enabled or not
    item["cpp-enable"] = value(true);
    item["java-enable"] = value(true);
    item["go-enable"] = value(true);
    item["python-enable"] = value(true);
    item["javascript-enable"] = value(true);
    item["typescript-enable"] = value(true);
    item["scheme-enable"] = value(true);
    // rendered code block i.e. ace editor is editable or not
    item["editable"] = value(true);
    // disable item or not
    item["disable-devtool-auto"] = value(false);
    item["disable-menu"] = value(false);
    item["clear-log"] = value(false);
    item["disable-select"] = value(false);
    item["disable-copy"] = value(false);
    item["disable-cut"] = value(false);
    item["disable-paste"] = value(false);
    // ACE editor cut/copy/paste
    item["ace-strict"] = value(false);
}

fn has_file(elem: &Option<&mut Array>, file: &str) -> bool {
    match elem {
        Some(elem) => elem.iter().any(|elem| match elem.as_str() {
            None => true,
            Some(s) => s.ends_with(file),
        }),
        None => false,
    }
}

fn insert_additional(doc: &mut DocumentMut, additional_type: &str, file: &str) {
    log::debug!("befor insert_additional:\n{}", doc.to_string());
    let doc = doc.as_table_mut();

    let empty_table = Item::Table(Table::default());
    let empty_array = Item::Value(Value::Array(Array::default()));
    let item = doc.entry("output").or_insert(empty_table.clone());
    let item = item
        .as_table_mut()
        .unwrap()
        .entry("html")
        .or_insert(empty_table);
    let array = item
        .as_table_mut()
        .unwrap()
        .entry(&format!("additional-{}", additional_type))
        .or_insert(empty_array);
    array
        .as_value_mut()
        .unwrap()
        .as_array_mut()
        .unwrap()
        .push(file);
    log::debug!("after insert_additional:\n{}", doc.to_string());
}




pub fn start_lang_server(hostname:&str, port:&str){
    log::info!("start language compiling server as daemon");
    
    // use the tmpfile find temporary directory
    let base_dir = env::current_dir().unwrap();
    let mut tmp = PathBuf::new();
    tmp.push(std::env::temp_dir().to_str().unwrap().to_string());

    log::debug!("temp path: {}\n", tmp.as_path().to_str().unwrap());
    
    let mut stdout_path = tmp.clone();
    stdout_path.push("mdbook-lang-server.out");

    let mut stderr_path = tmp.clone();
    stderr_path.push("mdbook-lang-server.err");

    let mut pid_path = tmp.clone();
    pid_path.push("mdbook-lang-server.pid");

    {
        let mut tmp = PathBuf::new();
        tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
        let mut pid_path = tmp.clone();
        pid_path.push("mdbook-lang-server.pid");

        let _ = fs::remove_file(pid_path);
    }
    let stdout = File::create(stdout_path).unwrap();
    let stderr = File::create(stderr_path).unwrap();

    let daemonize = Daemonize::new()
        .pid_file(pid_path.as_path()) // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory(base_dir.as_path()) // for default behaviour.          
        .umask(0o077) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/mdbook-lang-server.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/mdbook-lang-server.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            log::info!("Success, daemonized");
        }
        Err(e) => eprintln!("Error, {}", e),
    }
    log::info!("pid is:{}", std::process::id());

    let mut signals = Signals::new([SIGINT, SIGTERM, SIGQUIT]).unwrap();

    thread::spawn(move || {
        for sig in signals.forever() {
            log::info!("Received signal {:?}", sig);
            let mut tmp = PathBuf::new();
            tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
            let mut pid_path = tmp.clone();
            pid_path.push("mdbook-lang-server.pid");

            let _ = fs::remove_file(pid_path);
            process::exit(-1);
        }
    });

    let result = lang_server(hostname, port);
    let mut tmp = PathBuf::new();
    tmp.push(std::env::temp_dir().to_str().unwrap().to_string());
    let mut pid_path = tmp.clone();
    pid_path.push("mdbook-lang-server.pid");

    let _ = fs::remove_file(pid_path);
    
    match result{
        Ok(_code) => {
            log::info!("return from axum serve");
            // process::exit(code);
        }
        Err(e) =>{
            log::debug!("start language compiling server error:{}\n", e);
            process::exit(-2);
        }
    }
}

// as a daemon that serving the language RestFull API request
#[tokio::main]
async fn lang_server(hostname:&str, port:&str) -> Result<i32, std::io::Error>{
    log::debug!("Starting server on {}:{}", hostname, port);
    //the compiling service supports cross domain request
    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
//    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_origin(Any)
    .allow_headers([CONTENT_TYPE]);

    // Define Routes
    let app = Router::new()
        .route("/", get(|| {
            println!("response :{}", "working ... ");
            async {"Hello, mdbook-lang multi-language programming server!"}
        }))
        .route("/api/v1/build-code", post(build_code))
        .layer(cors);  //support cross domain restful api calling

    let listener = tokio::net::TcpListener::bind(format!("{}:{}",hostname, port))
        .await;
    match listener{
        Ok(listener) => {
            log::info!("listening on {}", listener.local_addr().unwrap());
            match axum::serve(listener, app).await{
                Ok(_) => {
                    log::info!("axum::serve await ok\n");
                }
                Err(e)=>{
                    log::info!("axum::serve error:{}\n", e);
                }
            }
            log::info!("listening on exit for language compiling server");
            return Ok(0);
        }
        Err(e) => {
            return Err(e);
        }
    }
}


pub async fn build_code(Json(code): Json<Code>) -> String {
    // load the sandbox configuration
    let sandbox_cmd = std::env::var("MDBOOKLANG_SERVER_SANDBOX_CMD");
    let mut sandbox_args_vec:Vec<String> = vec![];
    if let Ok(cmd) = sandbox_cmd{
        sandbox_args_vec.push(cmd);
        let sandbox_args = std::env::var("MDBOOKLANG_SERVER_SANDBOX_ARGS").unwrap_or_else(|_| "".to_string());   
        sandbox_args.split(':')
        .for_each(|arg| sandbox_args_vec.push(arg.to_string()));
    }
    
    let result = match code.lang.as_str(){
        "cpp" => build_cpp(code.code_block, sandbox_args_vec),
        "c" => build_c(code.code_block, sandbox_args_vec),
        "java" => build_java(code.code_block, sandbox_args_vec),
        "python" => build_python(code.code_block, sandbox_args_vec),
        "go" => build_go(code.code_block, sandbox_args_vec),
        "javascript" => build_javascript(code.code_block, sandbox_args_vec),
        "typescript" => build_typescript(code.code_block, sandbox_args_vec),
        "scheme" => build_scheme(code.code_block, sandbox_args_vec),
        _ => build_cpp(code.code_block, sandbox_args_vec),
    };
    use serde_json::json;
    let result = serde_json::to_string(&json!({
        "result": result
    })).unwrap();

    result
}

