extern crate github_rs;
extern crate serde_json;
extern crate hyper;
use serde_json::{Value, Error};
use github_rs::client::Github;
use std::process::Command;
fn get_nth_argument(n:usize) -> Result<String, &'static str>{
    for arg in std::env::args().skip(n){
        return Ok(arg);
    }
    Err("No token was provided")
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
            match repos {
                Ok((headers, status, json)) => {
                    match status{
                        hyper::StatusCode::Unauthorized => {
                            println!("Invalid token entered!");
                        }
                        _ => {
                            if let Some(json) = json{
                                if(json.is_array()){
                                    println!("Is array!");
                                    let repo_vec = json.as_array();
                                    println!("Array: {:?}", repo_vec);
                                    for repo_item in repo_vec{
                                        for repo_attribute in repo_item{
                                            if(repo_attribute.is_object()){
                                                if let Some(html_obj) = repo_attribute.get("html_url"){
                                                    if let Some(html) = html_obj.as_str(){
                                                        println!("Downloading {}", html);
                                                        let protocol:String = html.chars().take(8).collect();
                                                        let url:String = html.chars().skip(8).collect();
                                                        let clone_url = format!("{}{}@{}",protocol,token,url);
                                                        Command::new("git")
                                                            .arg("clone")
                                                            .arg(clone_url)
                                                            .spawn()
                                                            .expect("failed to execute command")
                                                            .wait();
                                                    }
                                                    
                                                } 
                                            }
                                        }

                                    }
                                }
                            }                            
                        }
                    }

                },
                Err(e) => println!("{}", e)
            }            
        }
        Err(e) => println!("{}", e)
    }
}