use clap::Parser;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

const BASE_URL: &str = "https://api.apileague.com/retrieve-random-meme";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    keyword_to_search: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meme {
    description: String,
    url: String,
    #[serde(alias = "type")]
    meme_type: String,
    width: u16,
    height: u16,
    ratio: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let Ok(api_key) = env::var("API_LEAGUE_API_KEY") else {
        panic!("API key could not be loaded correctly from the environmental variables.");
    };
    let arg = Args::parse();
    let params = [
        ("keywords", &arg.keyword_to_search),
        ("api-key", &api_key),
        ("media-type", &String::from("image/jpeg")),
    ];
    let client = reqwest::Client::new();
    let content: Meme = client
        .get(BASE_URL)
        .query(&params)
        .send()
        .await?
        .json()
        .await?;

    println!("Here is your content: {content:?}");

    Ok(())
}
