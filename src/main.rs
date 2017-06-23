extern crate github_rs;
extern crate serde_json;
extern crate hyper;
extern crate terminal_size;
use serde_json::Value;
use github_rs::client::Github;
use std::process::Command;
use terminal_size::{Width, terminal_size};

struct Config{
    token:String,
    output_location:String,
}

impl Config{
    pub fn new() -> Config{
        Config {
            token: String::new(),
            output_location: String::from("repositories")
        }
    }
    pub fn new_with_token(token:String) -> Config{
        let mut config:Config = Config::new();
        config.token = token;
        config
    }
    pub fn new_with_token_and_output_dir(token:String, output_location:String) -> Config{
        let mut config:Config = Config::new();
        config.token = token;
        config.output_location = output_location;
        config
    }

}


fn get_nth_argument(n:usize) -> Result<String, &'static str>{
    for arg in std::env::args().skip(n){
        return Ok(arg);
    }
    Err("No token was provided")
}

fn get_width_of_terminal() -> usize{
    let size = terminal_size();
    if let Some((Width(w),_)) = size {
        return w as usize;
    }
    0 as usize
}

fn process_repo_json(repo_json:Option<Value>, config:Config){
    if let Some(json) = repo_json{
        if json.is_array() {
            if let Some(repository_list) = json.as_array(){
                for repository in repository_list{
                    //println!("Repository: {:?}", repository);
                    if repository.is_object(){

                        let repository_html_url = repository.get("html_url").unwrap().as_str().unwrap();
                        let repository_name = repository.get("name").unwrap().as_str().unwrap();
                        println!("Downloading {}", repository_html_url);
                        
                        let protocol:String = repository_html_url.chars().take(8).collect();
                        let url:String = repository_html_url.chars().skip(8).collect();
                        let clone_url = format!("{}{}@{}",protocol,config.token,url);
                        let result = Command::new("git")
                            .arg("clone")
                            .arg(clone_url)
                            .arg(format!("{}/{}", &config.output_location, repository_name))
                            .spawn()
                            .expect("failed to execute command")
                            .wait();
                        println!("{:?}", result);
                        println!("{}", "-".repeat(get_width_of_terminal()));
                    }

                }
            }
        }
    }      
}

fn process_repos(repos:Result<(hyper::Headers, hyper::StatusCode, std::option::Option<Value>), github_rs::errors::Error>,
    config:Config){
    match repos {
        Ok((_, status, json)) => {
            match status{
                hyper::StatusCode::Unauthorized => {
                    println!("Invalid token entered!");
                }
                _ => {
                    process_repo_json(json,config);                      
                }
            }

        },
        Err(e) => println!("{}", e)
    }  
}

fn main() {
    let arg_tuple=(get_nth_argument(1), get_nth_argument(2));
    let config:Config;
    //println!("{:?}", arg_tuple);
    match arg_tuple{
        (Ok(tok), Ok(dir)) => config=Config::new_with_token_and_output_dir(tok, dir),
        (Ok(tok), _) => config=Config::new_with_token(tok),
        _ => {
            println!("Token was not supplied!");
            std::process::exit(1);
        }
    }
    let client = Github::new(&config.token).unwrap();
    let repos = client.get()
                .user()
                .repos()
                .execute();
    process_repos(repos,config);
}