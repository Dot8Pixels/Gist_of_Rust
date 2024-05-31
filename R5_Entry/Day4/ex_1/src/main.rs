#[derive(Debug)]
enum AnimalType {
    Cat,
    Duck,
}

// How to return string or &str from enum.
impl AnimalType {
    fn as_str(&self) -> &str {
        match self {
            AnimalType::Cat => "🐈",
            AnimalType::Duck => "🐥",
        }
    }
}

// How to use type as a parameters, hey!👇 what's this? 😳
fn say(animal_type: AnimalType) -> &'static str {
    // To survive from fn {}, we need 👆 'static to let is has program's lifetime.
    match animal_type {
        AnimalType::Cat => "meaowww",
        AnimalType::Duck => "quackkk",
    }
}

fn main() {
    println!(
        "{0:?} aka {1:?} say {2:?}",
        AnimalType::Cat,
        AnimalType::Cat.as_str(),
        say(AnimalType::Cat)
    );
    println!(
        "{0:?} aka {1:?} say {2:?}",
        AnimalType::Duck,
        AnimalType::Duck.as_str(),
        say(AnimalType::Duck)
    );
}
