// We need👇 <'a> here.
struct Me<'a> {
    name: &'a str, // Because of this 'a.
                   // Mean 👆 this str name is have a good life in this { } scope.
}

// So 👇 we will need <'a> here too when we impl! 🤷
impl<'a> Me<'a> {
    // Due to👆 this.
    fn say_my_name(&self) -> &str {
        self.name
    }
}

// But this don't.
struct You {
    name: String, // Because of no 'a here, why?
                  // Because 👆 String, Vec, Box allocated on heap. Thanks heap!
}

// So this no need <'a>.
impl You {
    fn say_my_name(&self) -> String {
        self.name.to_owned()
    }
}

// And this also don't need <'a>
struct Cat {
    name: &'static str, // Because it's a long life static.
}

// So this no need <'a>.
impl Cat {
    fn say_my_name(&self) -> &str {
        self.name
    }
}

fn main() {
    // Actually we need 'a 👇 lifetime annotations. 😱
    fn hello_with_lifetime<'a>(x: &'a str) -> &'a str {
        x
    }

    // Or this... 😱
    fn hello_with_any_lifetime(x: &'_ str) -> &'_ str {
        x
    }

    // Good news, we can do this instead (Thanks to compiler!) 🙏
    fn hello_str(x: &str) -> &str {
        x
    }

    // Anyway for multiple params, we not sure how long lifetime each one.
    // So this 👇 and  👇 also here 👇 and here 👇 will need. 😅
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // This └───────┴───────────┴───────────┘ have same lifetime
        // which defined as 'a (can be any e.g. 'foo, 'lol).

        if x.len() > y.len() {
            x // Maybe return this
        } else {
            y // Maybe return this
        }
    }

    println!("1️⃣ {:?}", hello_with_lifetime("world"));
    println!("2️⃣ {:?}", hello_with_any_lifetime("world"));
    println!("3️⃣ {:?}", hello_str("world"));

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("3️⃣ The longest string is {}", result);
    }

    // Return 'a lifetime.
    fn longest_a<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
    // where clause should look like this.
    where
        'b: 'a, // 'b must outlive the lifetime of 👆 'a
    {
        if x.len() > y.len() {
            x // 'a
        } else {
            y // 'b
        }
    }

    println!("The longest string is {}", longest_a("foooooo", "bar"));
    println!("The longest string is {}", longest_a("foo", "barrrrr"));

    // Return 'b lifetime.
    fn longest_b<'a, 'b>(x: &'a str, y: &'b str) -> &'b str
    // where clause should look like this.
    where
        'a: 'b, // 'a must outlive the lifetime of 👆 'b
    {
        if x.len() > y.len() {
            x // 'a
        } else {
            y // 'b
        }
    }

    println!("The longest string is {}", longest_b("foooooo", "bar"));
    println!("The longest string is {}", longest_b("foo", "barrrrr"));

    // Say my name
    println!("{:?}", Me { name: "foo" }.say_my_name());

    // To &str → String You have to add 👇 to_owned.
    println!(
        "{:?}",
        You {
            name: "bar".to_owned()
        }
        .say_my_name()
    );

    // Say my name 🎵
    println!("{:?}", Cat { name: "baz" }.say_my_name());
}
