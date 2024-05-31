fn main() {
    // Start with str
    let foo_str = "foo"; // &str 👈 Reference to a string slice.

    // Try move str
    let bar_str = foo_str;
    println!("bar_str: {bar_str}");
    println!("foo_str: {foo_str}");

    // Now let's try String
    let foo_string = foo_str.to_string(); // String 👈 So we can move it.

    // Try move String.
    let bar_string = foo_string;
    println!("bar_string: {bar_string}");

    // But foo_string is already moved. 💀
    // 😱 You can try uncomment 👇 this to see an error.
    // println!("foo_string:{foo_string}");
    //                      ^^^^^^^^^^^^ value borrowed here after move

    // So we need & to make a reference.
    // 1️⃣ let other borrow `&` instead of move.
    let borrowed_bar_string = &bar_string;
    println!("bar_string: {bar_string}"); // Still can access.
    println!("borrowed_bar_string: {borrowed_bar_string}"); // Also here.

    // 2️⃣ or make a clone/copy instead of move.
    let borrowed_bar_string = bar_string.clone();
    println!("bar_string: {bar_string}"); // Still can access.
    println!("borrowed_bar_string: {borrowed_bar_string}"); // Also here.
    
    let bar_str = bar_string.as_str();

    println!("bar_string: {bar_string}");
    println!("bar_str: {bar_str}");

    // &str → String
    assert_eq!(bar_string, foo_str.to_string());
}
