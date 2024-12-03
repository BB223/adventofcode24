use std::{env, fs, path::Path};

use dotenvy::dotenv;
use reqwest::Result;

async fn get_input_web(day: u32) -> Result<String> {
    dotenv().expect(".env not found");
    let client = reqwest::Client::new();
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    println!("Requesting... {}", url);

    client
        .get(url)
        .header(
            "cookie",
            format!("session={}", env::var("SESSION_COOKIE").unwrap()),
        )
        .send()
        .await?
        .text()
        .await
}

pub async fn get_input(day: u32) -> String {
    let path_str = env::var("AOC_PATH").unwrap_or("../aocfiles/".to_string()) + &day.to_string();
    let path = Path::new(&path_str);
    if path.exists() {
        println!("Reading File... {}", path_str);
        fs::read_to_string(path).unwrap()
    } else {
        let input = get_input_web(day).await.unwrap();
        println!("Write File... {}", path_str);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, input.clone()).unwrap();
        input
    }
}
