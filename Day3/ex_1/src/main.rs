fn main() {
    // 😭 Before: we did use `Tuple`.
    let animal = (("name", "foo"), ("age", 42));
    println!("{0:?}: {1:?}", animal.0 .0, animal.0 .1); // 😭 So hard to access tuple!
    println!("{0:?}: {1:?}", animal.1 .0, animal.1 .1); // 😭 Stop this!

    // 😚 After: we use `Struct` instead.
    struct Animal {
        name: String, // We use `String` here not &str (will talk about this later).
        age: u8,      // `u8` mean unsigned integer (2^8 − 1) = 255
    }

    // Create animal
    let animal = Animal {
        name: "foo".to_owned(),// You can also use `to_string()` here.
        age: 42u8,              // Shorthand for casting `42 as u8` or `42_u8`.
    };

    println!("name: {:?}", animal.name); // 😚 So easy to use!
    println!("age: {:?}", animal.age);
}
