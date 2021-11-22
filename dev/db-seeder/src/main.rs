use std::fs::File;
use std::io::BufReader;

use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{doc, Document};
use serde_json;
use serde_json::{ Value };
use std::collections::HashMap;


async fn seed_users(production:&Database) -> Result<(), Box<dyn std::error::Error>>  {
    
    let names_reader = BufReader::new(File::open("data/names.json")?);    
    let names:Vec<HashMap<String, String>> = serde_json::from_reader(names_reader)?;
    
    let mut docs = vec!();
        
    for name in names {
        let fname = &name["first_name"];
        let lname = &name["last_name"];

        docs.push(
            doc!{
                "first_name": fname,
                "last_name": lname,
                "email": format!("{}.{}@mail.com", fname, lname),
                "last_login": "2021-11-19T00:00:00+00:00"
            }
        )
    }

    production.collection("users").insert_many(docs, None).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        
    let client = Client::with_uri_str("mongodb://admin:admin@127.0.0.1:27017").await?;    
    let production = client.database("production");
    //let users = production.collection("users");    

    seed_users(&production).await?;    

    Ok(())
}
