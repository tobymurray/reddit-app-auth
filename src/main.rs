use std::collections::HashMap;
use serde::Deserialize;
use std::fmt;
use structopt::StructOpt;

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

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Get all possible scopes")]
    GetScopes {
        verbose: Option<bool>
    },

    #[structopt(about = "Generate")]
    Generate {
        #[structopt(short = "i", long = "client-id")]
        client_id: String,
    
        #[structopt(default_value = "https://www.reddit.com/prefs/apps", short = "r", long = "redirect-url")]
        redirect_uri: String,
        
        #[structopt(default_value = "temporary", short = "d", long = "duration")]
        duration: String,
    
        #[structopt(short = "s", long = "scopes")]
        scopes: String
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // client_id
    // redirect_uri
    // duration, default to "temporary"
    // scope
    let args = Cli::from_args();

    match args.cmd {
        Some(cmd) => println!("Command passed"),
        Node => println!("No command!"),
    }

    // println!("{:?}", );

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


    // let response = reqwest::blocking::get("https://www.reddit.com/api/v1/scopes")?.json::<HashMap<String, Scope>>().unwrap();

    // for (_, scope_body) in response.iter() { 
    //     println!("{}", scope_body);
    // }
    Ok(())
}