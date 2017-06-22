extern crate github_rs;
use github_rs::client::Github;

fn getNArgument(n:usize) -> Result<String, &'static str>{
    for arg in std::env::args().skip(n){
        return Ok(arg);
    }
    Err("No token was provided")
}

fn main() {
    
    match getNArgument(1){
        Ok(tok) => {
            let token:String = tok;
            let client = Github::new(token).unwrap();
            let me = client.get()
                        .user()
                        .execute();
            match me {
                Ok((headers, status, json)) => {
                    println!("{}", headers);
                    println!("{}", status);
                    if let Some(json) = json{
                        println!("{}", json);
                    }
                },
                Err(e) => println!("{}", e)
            }            
        }
        Err(e) => println!("{}", e)
    }
}