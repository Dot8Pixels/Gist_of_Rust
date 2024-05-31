// 👇 Let's move struct out from `fn main`.
#[derive(Debug, Clone)] // Derive Debug so we can print later.
struct Animal {
    #[allow(dead_code)] // Allow dead code.
    name: String,

    #[allow(dead_code)]
    age: u8,

    // 👇 `type` is reserved word but we still can use it.
    r#type: String, // r# mean raw string.
}

// We will implement some method for Animal.
impl Animal {
    // The `new` constructor return 👇 itself call `Self`.
    fn new(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            r#type: "duck".to_owned(),
        }
    }

    // `new_cat` alternative constructor with default type.
    fn new_cat(name: &str, age: u8) -> Self {
        Animal {
            name: name.to_owned(),
            age,
            r#type: "cat".to_owned(),
        }
    }

    // Define static method.
    pub fn static_say(animal_type: &str) -> &str {
        match animal_type {
            // 👇 This &str is bad practice, we will need enum here (later).
            "cat" => "meaowww",
            "duck" => "quackkk",
            _ => "wat!?",
        }
    }

    // With &self 👇 method.
    pub fn say(&self) -> &str {
        // So we can call 👇 ourself here.
        let animal_type = self.r#type.as_str();
        Animal::static_say(animal_type)
    }
}

fn main() {
    // So we can call new 👇 like this.
    let animal = Animal::new("foo", 42u8);
    println!("animal: {:#?}", animal);

    // Call say via static method.
    let static_say_str = Animal::static_say("duck");
    println!("static_say_str: {:#?}", static_say_str);

    // Also can new cat 👇 like this.
    let cat = Animal::new_cat("bar", 24u8);
    println!("cat: {:#?}", cat);

    // Call say via method itself.
    let say_str = cat.say();
    println!("say_str: {:#?}", say_str);

    // Or via Animal 😳
    println!("Animal::say: {:#?}", Animal::say(&cat));

    // You can also clone after derive Clone above 👆
    let mut duck = cat.clone();
    duck.name = "duck the duck".to_owned();
    duck.age = 13;

    //  Destructing from struct.
    let Animal { age, .. } = cat;
    println!("age: {:#?}", age);

    //  Match struct where animal
    match &duck {
        // Match age at 24
        Animal { age: 24, .. } => println!("match age at 24 : {:#?}", age),

        // Match age between 30-50 range.
        Animal { age: 30..=50, .. } => println!("match age between 30-50 : {:#?}", age),

        // Guard name equal to "foo"
        Animal { name, .. } if name == "duck the duck" => println!("animal.name: {:#?}", name),

        // Other age.
        _ => println!("age not in range"),
    }
}
