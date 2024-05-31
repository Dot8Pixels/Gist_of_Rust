// ...Continue from previous example.
#[derive(Debug, Clone)]
struct Human {}
struct Animal {}

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

use enum_dispatch::enum_dispatch;
use rand::Rng;

#[enum_dispatch]
enum SayableEnum {
    Animal,
    Human,
}

impl Into<SayableEnum> for Animal {
    fn into(self) -> SayableEnum {
        SayableEnum::Animal(self)
    }
}

impl Into<SayableEnum> for Human {
    fn into(self) -> SayableEnum {
        SayableEnum::Human(self)
    }
}

fn animal_or_human() -> SayableEnum {
    if rand::thread_rng().gen_range(0u8..9u8) % 2 == 0 {
        Animal {}.into()
    } else {
        Human {}.into()
    }
}
fn main() {
    // Randomly say by animal or human.
    println!(
        "{:?}",
        match animal_or_human() {
            SayableEnum::Animal(e) => e.say(),
            SayableEnum::Human(e) => e.say(),
        }
    );
}
