use dotenv::dotenv;
use std::env;

const BASE_URL : &str = "https://api.apileague.com/retrieve-random-meme";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let Ok(api_key) = env::var("API_LEAGUE_API_KEY") else {
        panic!("API key could not be loaded correctly from the environmental variables.");
    };
    println!("Here is your api key: {}", api_key);
    Ok(())

}
