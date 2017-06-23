extern crate github_rs;
extern crate serde_json;
extern crate hyper;
extern crate terminal_size;
use serde_json::Value;
use github_rs::client::Github;
use std::process::Command;
use terminal_size::{Width, Height, terminal_size};

fn get_nth_argument(n:usize) -> Result<String, &'static str>{
    for arg in std::env::args().skip(n){
        return Ok(arg);
    }
    Err("No token was provided")
}

fn get_width_of_terminal() -> usize{
    let size = terminal_size();
    if let Some((Width(w),Height(h))) = size {
        return w as usize;
    }
    0 as usize
}

fn process_repo_json(repo_json:Option<Value>, token:String){
    if let Some(json) = repo_json{
        if json.is_array() {
            if let Some(repository_list) = json.as_array(){
                for repository in repository_list{
                    //println!("Repository: {:?}", repository);
                    if(repository.is_object()){
                        if let Some(html_obj) = repository.get("html_url"){
                            if let Some(html) = html_obj.as_str(){
                                println!("Downloading {}", html);
                                
                                let protocol:String = html.chars().take(8).collect();
                                let url:String = html.chars().skip(8).collect();
                                let clone_url = format!("{}{}@{}",protocol,token,url);
                                let result = Command::new("git")
                                    .arg("clone")
                                    .arg(clone_url)
                                    .spawn()
                                    .expect("failed to execute command")
                                    .wait();
                                println!("{}", "-".repeat(get_width_of_terminal()));
                            }
                            
                        } 
                    }

                }
            }
        }
    }      
}

fn process_repos(repos:Result<(hyper::Headers, hyper::StatusCode, std::option::Option<Value>), github_rs::errors::Error>,
    token:String){
    match repos {
        Ok((headers, status, json)) => {
            match status{
                hyper::StatusCode::Unauthorized => {
                    println!("Invalid token entered!");
                }
                _ => {
                    process_repo_json(json,token);                      
                }
            }

        },
        Err(e) => println!("{}", e)
    }  
}

fn main() {
    
    match get_nth_argument(1){
        Ok(tok) => {
            let token:String = tok;
            let client = Github::new(&token).unwrap();
            let repos = client.get()
                        .user()
                        .repos()
                        .execute();
          process_repos(repos,token);
        }
        Err(e) => println!("{}", e)
    }
}