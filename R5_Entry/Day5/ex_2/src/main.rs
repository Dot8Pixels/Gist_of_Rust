use async_trait::async_trait;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// 👇 We need this.
#[async_trait]
trait Animal {
    // 👇 Because of this async.
    async fn sleep(&self);
}

struct Cat;

// 👇 Also here.
#[async_trait]
impl Animal for Cat {
    async fn sleep(&self) {
        // Will sleep for 2 seconds.
        sleep(Duration::new(2, 0));
    }
}

// This `async fn main` need `tokio::main`.
#[tokio::main]
async fn main() {
    // Wait for sleepy cat for 2 sec.
    let now = SystemTime::now();
    Cat {}.sleep().await;

    // Ensure it's 2 sec.
    let now_sec = now.elapsed().ok().unwrap().as_secs();
    assert_eq!(now_sec, 2);
    println!("Cat has been sleep for {} sec.", now_sec);
}
