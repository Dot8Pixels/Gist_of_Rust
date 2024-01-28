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

use either::Either;
use rand::Rng;

// Can be either Sayable or Sayable.
fn animal_or_human() -> Either<impl Sayable, impl Sayable> {
    // Use random number instead of time, just for fun.
    if rand::thread_rng().gen_range(0u8..9u8) % 2 == 0 {
        Either::Left(Animal {})
    } else {
        Either::Right(Human {})
    }
}

fn main() {
    // Randomly say by animal or human.
    let either_animal_or_human = animal_or_human();
    println!(
        "{:?}",
        match either_animal_or_human.is_left() {
            true => either_animal_or_human.left().unwrap().say(),
            false => either_animal_or_human.right().unwrap().say(),
        }
    );
}
