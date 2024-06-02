use futures::future;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct AnimalData {
    id: String,
    #[serde(rename = "type")]
    animal_type: String,
    weight: f32,
    created_at: String,
}

// Shared client for each call.
async fn fetch_multiple_with_one_client_join_all(urls: &[&str]) -> anyhow::Result<Vec<AnimalData>> {
    // New shared client once.
    let client = Client::new();

    // How to use join_all.
    let results = future::join_all(urls.iter().map(|&url| {
        // Use shared client.
        let client = &client;
        async move {
            let resp = client.get(url).send().await?;
            resp.json::<AnimalData>().await
        }
    }))
    .await;

    // Return flattened results, silent if error.
    Ok(results
        // We use into_iter so we get Vec<AnimalData> instead of Vec<&AnimalData>
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

// Each client for each call.
async fn fetch_multiple_with_each_client_join_all(
    urls: &[&str],
) -> anyhow::Result<Vec<AnimalData>> {
    // How to use join_all.
    let results = future::join_all(urls.iter().map(|&url| async move {
        // Fetch each url with new client.
        reqwest::get(url).await?.json::<AnimalData>().await
    }))
    .await;

    // Return flattened results, silent if error.
    Ok(results
        // We use into_iter so we get Vec<AnimalData> instead of Vec<&AnimalData>
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}

fn merge_json(mut base: Value, addition: Value) -> Value {
    match (base, addition) {
        (Value::Object(mut base_map), Value::Object(add_map)) => {
            for (k, v) in add_map {
                base_map
                    .entry(k)
                    .and_modify(|base_val| {
                        *base_val = merge_json(base_val.take(), v.clone());
                    })
                    .or_insert(v);
            }
            Value::Object(base_map)
        }
        // If addition is not an object, just return addition.
        (_, addition) => addition,
    }
}

#[tokio::main]
async fn main() {
    let urls = [
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/30-fetch-multiple-futures/src/foo.json",
        "https://raw.githubusercontent.com/gist-rs/book/main/examples/r4/30-fetch-multiple-futures/src/bar.json"
    ];

    let fetched_data_one_client = fetch_multiple_with_one_client_join_all(&urls)
        .await
        .expect("Failed to fetch data");
    println!("fetch_multiple_with_one_client: {fetched_data_one_client:#?}");

    let fetched_data_each_client = fetch_multiple_with_each_client_join_all(&urls)
        .await
        .expect("Failed to fetch data");
    println!("fetch_multiple_with_each_client: {fetched_data_each_client:#?}");

    let current_dir = env::current_dir().expect("Cannot get current directory");

    let foo_filename: &str = "foo.json";
    let expected_foo_json_path = current_dir.join(foo_filename);
    let foo_json_content = fs::read_to_string(expected_foo_json_path).expect("Unable to read file");
    let expected_foo_data: AnimalData =
        serde_json::from_str(&foo_json_content).expect("Unable to parse json data to struct");
    println!("{:?}", expected_foo_data);

    let bar_filename: &str = "bar.json";
    let expected_bar_json_path = current_dir.join(bar_filename);
    let bar_json_content = fs::read_to_string(expected_bar_json_path).expect("Unable to read file");
    let expected_bar_data: AnimalData =
        serde_json::from_str(&bar_json_content).expect("Unable to parse json data to struct");
    println!("{:?}", expected_bar_data);

    // Combine the AnimalData instances into a vector
    let combined_expected_data = vec![expected_foo_data, expected_bar_data];
    println!("{:?}", combined_expected_data);

    // Assert that fetched data and combined expected data are equal
    assert_eq!(
        fetched_data_one_client, combined_expected_data,
        "Fetched data does not match the expected data"
    );
    assert_eq!(
        fetched_data_each_client, combined_expected_data,
        "Fetched data does not match the expected data"
    );
}
