use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalType {
    #[strum(serialize = "cat", to_string = "catty")]
    Cat,
    #[strum(serialize = "duck", to_string = "ducky")]
    Duck,
    Unknown,
    #[strum(disabled)]
    Pet(String),
}

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum AnimalSound {
    #[strum(serialize = "cat", to_string = "meaowww")]
    Cat,
    #[strum(serialize = "duck", to_string = "quackkk")]
    Duck,
}

fn main() {
    // Get AnimalType from &str.
    let animal_type = AnimalType::from_str("cat");
    println!("1 animal_type: {animal_type:?}");

    // Unwrap or assign as Unknown.
    let animal_type = animal_type.unwrap_or(AnimalType::Unknown).to_string();
    println!("2 animal_type: {animal_type:?}");

    // Get AnimalSound from str.
    let cat_sound_result = AnimalSound::from_str("cat");
    println!("3 cat_sound_result: {:?}", cat_sound_result);

    // Handle cat_sound Result.
    let cat_sound_string = match cat_sound_result {
        // Handle happy case.
        Ok(animal_sound) => animal_sound.to_string(),

        // Handle error case.
        Err(err) => panic!("{:?}", err),
    };

    println!("4 cat_sound_string: {cat_sound_string:?}");

    // 😱 Uncomment this to experience an error and try to fix it by add Clone, Copy to AnimalSound
    // println!("4️⃣ cat_sound_result: {cat_sound_result:?}");

    // Match
    let animals = vec![AnimalType::Cat, AnimalType::Pet("snoopy".to_owned())];
    let my_pet = animals
        .into_iter()
        .filter_map(|e| match e {
            AnimalType::Pet(name) => Some(name),
            AnimalType::Cat => None,
            AnimalType::Duck => None,
            AnimalType::Unknown => None,
        })
        .collect::<Vec<_>>();

    println!("5 my_pet: {:?}", my_pet.join(","));
}
