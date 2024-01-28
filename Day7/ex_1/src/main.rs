// ...Continue from previous example.
#[derive(Debug, Clone)]
struct Animal {}
struct Human {}

trait Sayable {
    fn say(&self) -> String;
}

impl Sayable for Animal {
    fn say(&self) -> String {
        "meow!".to_owned()
    }
}

impl Sayable for Human {
    fn say(&self) -> String {
        "hi!".to_owned()
    }
}

// Compiler'll need this👇 Box to know its size (Box's size).
fn animal_or_human() -> Box<dyn Sayable> {
    // Compiler'll need this 👆 dyn to know it'll be dynamic (Animal or Human)

    // How to get current time.
    let now: std::time::SystemTime = std::time::SystemTime::now();

    // How to get duration since UNIX_EPOCH.
    let result_duration: Result<std::time::Duration, std::time::SystemTimeError> =
        now.duration_since(std::time::UNIX_EPOCH);

    // How to convert : Option<std::time::Duration>`Result` to `Option`.
    let maybe_duration: Option<std::time::Duration> = result_duration.ok();

    match maybe_duration {
        Some(duration) => {
            // Take secs
            let sec: u64 = duration.as_secs();

            // Modulo so we get 50% chance randomly by current time.
            if sec % 2 == 0 {
                Box::new(Animal {})
            } else {
                Box::new(Human {})
            }
        }
        // When you not finish implementation yet, try use todo!.
        None => todo!(),
    }
}

fn main() {
    // Randomly say by animal or human.
    println!("{:?}", animal_or_human().say());
}
