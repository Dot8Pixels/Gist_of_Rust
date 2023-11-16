use std::collections::HashMap; // `use` aka `import` in js.
// We talk about :: 👆 already, it's just a separator.

fn main() {
    // Create new mutable hashmap
    let mut foo_hashmap = HashMap::new(); // Yet another :: here.

    // It's mutable so we can update it
    foo_hashmap.insert("name", "foo");
    foo_hashmap.insert("age", "42");
    println!("foo_hashmap insert: {foo_hashmap:#?}"); 

    // Or rather use HashMap::from for batch insert.
    let mut foo_hashmap = HashMap::from([
        ("name", "foo"),
        ("age", "42"),
    ]);
    println!("foo_hashmap batch: {foo_hashmap:#?}"); 


    // 1️⃣ And when we tend to throw an error if not exist.
    let _name_or_error = foo_hashmap.get("name").expect("Expect 'name'");        // Will return &&str
    let name_or_error = foo_hashmap.get::<str>("name").expect("Expect 'name'"); // Will return &&str

    println!("1 name_or_error:{name_or_error:?}");

    // 2️⃣ Now use it in varies style.
    let _maybe_name = foo_hashmap.get("name");          // Will return `Option<&&str>`.
    let _maybe_name = foo_hashmap.get("name").copied(); // Will return `Option<&str>`.
    let maybe_name = foo_hashmap.get::<str>("name");   // Will return `Option<&&str>`.

    // `match` aka `switch` in js.
    // Let's handle `Option<&&str>` which can be `Some` or `None`.
    match maybe_name {
        Some(name) => println!("2 hello {name}",),// Will print "hello foo".
        None => panic!("who!?"),                   // Will throw error with `panic!` macro.
    };

    // 3️⃣ Or handle with `unwrap_or`.
    let unwrapped_name = maybe_name.unwrap_or(&"who!?");

    // And assign back by return after matched.
    let hi = match unwrapped_name {
        &"foo" => format!("3 unwrapped_name:{unwrapped_name}"), // Will return unwrapped_name.
        _ => panic!("who!?"),                                    // `_` aka `default` in js.
    };

    println!("{hi}");

    // 4️⃣ Let's iterate and print it out.
    foo_hashmap
        .iter()                               // iter as usual, will use `for_each`.
        .for_each(|e| println!("4 {:?}", e)); // Just print, No need to collect.

    // 5️⃣ Then we will use get👇 to borrow the value.
    let name = foo_hashmap.get("name").unwrap();
    println!("5 unwrap_name:{name}");

    // 6️⃣ Or take it by remove 👇.
    let age = foo_hashmap.remove("age").unwrap();
    println!("6 remove_age:{age}");

    // 😱 So this will fail because we already remove it above.
    // let _age = foo_hashmap.remove("age").unwrap();
}
