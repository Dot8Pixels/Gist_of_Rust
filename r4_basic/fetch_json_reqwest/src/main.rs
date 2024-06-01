use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
// This will auto convert form `camelCase` to `snake_case`
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    #[serde(rename = "type")]
    animal_type: String,
    weight: f32,
    // This will auto convert form "createdAt" to `created_at`
    created_at: String,
}

// 👇 How to async fetch.   // 👇 How to use anyhow.
async fn fetch(url: &str) -> anyhow::Result<Vec<AnimalData>> {
    // Beware, This will new client every fetch.
    let json = reqwest::get(url)
        // Use `?` to unwrap fetch Result and return Error to anyhow above if has
        .await?
        // Parse to vec of AnimalData.
        .json::<Vec<AnimalData>>()
        // Use `?` to unwrap json parse Result and return Error to anyhow above if has
        .await?;

    Ok(json)
}

// How to async main.
#[tokio::main]
async fn main(){
    let json = fetch("https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/20-fetch-json-reqwest/src/foo.json").await;
    println!("Response data: {json:#?}");

    let current_dir = env::current_dir().expect("Cannot get current directory");

    let filename = "expected_response.json";
    let expected_json_path = current_dir.join(filename);


    let json_content = fs::read_to_string(expected_json_path).expect("Unable to read file");
    let expected_data: Vec<AnimalData> = serde_json::from_str(&json_content).expect("Unable to parse json data to struct");

    assert_eq!(json.unwrap(), expected_data);   
}
