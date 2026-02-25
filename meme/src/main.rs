use clap::Parser;
use dotenv::dotenv;
use reqwest;
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

fn load_key() -> String {
    dotenv().ok();
    let Ok(api_key) = env::var("API_LEAGUE_API_KEY") else {
        panic!("API key could not be loaded correctly from the environmental variables.");
    };
    api_key
}

async fn get_picture(url: String) -> String {
    reqwest::Client::new()
        .get(url)
        .send()
        .await
        .expect("Could not retrieve picture url.")
        .text()
        .await
        .unwrap()
}


async fn get_meme(kw : String, key : String) -> Meme {
    let params = [
        ("keywords", kw),
        ("api-key", key),
        ("media-type", String::from("image/jpeg")),
    ];
    reqwest::Client::new()
        .get(BASE_URL)
        .query(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let arg = Args::parse();
    let meme_content = get_meme(arg.keyword_to_search, load_key()).await;
    println!("Here is the response from the picture url: {}", get_picture(meme_content.url).await);
    Ok(())
}
