use std::collections::HashMap;
use serde::Deserialize;
use std::fmt;
use structopt::StructOpt;
use reqwest::blocking;

#[derive(Deserialize)]
struct Scope {
    description: String,
    id: String,
    name: String
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<16}  {:<64}  {}", self.id, self.name, self.description)
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scope")
         .field("name", &self.name)
         .field("description", &self.description)
         .finish()
    }
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Get all possible scopes")]
    GetScopes {
        #[structopt(short = "v", long = "verbose")]
        verbose: Option<bool>
    },
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>,

    #[structopt(short = "i", long = "client-id")]
    client_id: String,

    #[structopt(default_value = "https://www.reddit.com/prefs/apps", short = "r", long = "redirect-url")]
    redirect_uri: String,
    
    #[structopt(default_value = "temporary", short = "d", long = "duration")]
    duration: String,

    #[structopt(short = "s", long = "scopes")]
    scopes: String
}

fn parse_scopes(response: blocking::Response) -> HashMap<std::string::String, Scope> {
    response.json::<HashMap<String, Scope>>()
        .unwrap()
}

fn print_scopes(command: Command) {
    println!("Printing scopes!");
    let response = reqwest::blocking::get("https://www.reddit.com/api/v1/scopes");
    let response = match response {
        Ok(body) => parse_scopes(body),
        Err(e) => panic!(e)
    };
    
    for (_, scope_body) in response.iter() { 
        println!("{}", scope_body);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // client_id
    // redirect_uri
    // duration, default to "temporary"
    // scope
    
    let args = Cli::from_args();
    return Ok(());
    
    println!("{:#?}", args);
    println!("Going to try and match the args");
    // match args.cmd {
    //     Some(cmd) => print_scopes(cmd),
    //     None => { println!("no command provided");},
    // }

    // let response_type = "code";
    // let state = "asdf"; // random string
    // let url = format!("https://www.reddit.com/api/v1/authorize?\
    //     client_id={}&\
    //     response_type={}&\
    //     state={}&\
    //     redirect_uri={}&\
    //     duration={}&\
    //     scope={}", args.client_id, response_type, state, args.redirect_uri, args.duration, args.scopes);
    // println!("Making request to: {}", url);

    // println!("{} ")

    Ok(())
}