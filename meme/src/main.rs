use dotenv::dotenv;
use std::env;

const BASE_URL : &str = "https://api.apileague.com/retrieve-random-meme";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let Ok(api_key) = env::var("API_LEAGUE_API_KEY") else {
        panic!("API key could not be loaded correctly from the environmental variables.");
    };
    let params = [("keywords", "cat"), ("api-key", &api_key), ("media-type", "image/jpeg")];
    let client = reqwest::Client::new();
    let content = client.get(BASE_URL).query(&params).send().await?.text().await?;
        
    println!("Here is the response: {content:?}");
    Ok(())

}
