use std::ops::Deref;

// Do this
fn good(foo_var: &str) {
    println!("{foo_var}");
}

// Not this
fn bad(foo_var: &String) {
    println!("{foo_var}");
}

fn good_and_bad_example() {
    // 🤩 Deref coercion happen here.

    // "Ferris".to_string() creates a String.
    // &"Ferris".to_string() creates a reference to a String (&String).
    // good expects a &str, so Rust performs deref coercion: &String is converted to &str.
    println!("{:?}", good(&"Ferris".to_string())); // &String → &str

    // "Ferris" is a &str.
    // &"Ferris" creates a reference to a &str (&&str).
    // good expects a &str, so Rust dereferences the &&str to &str.
    println!("{:?}", good(&"Ferris")); // &&str → &str

    // "Ferris" is already a &str.
    // good expects a &str, so no conversion is needed.
    println!("{:?}", good("Ferris")); // &str → &str

    // "Ferris".to_string() creates a String.
    // &"Ferris".to_string() creates a reference to a String (&String).
    // bad expects a &String, so this call works without coercion.
    println!("{:?}", bad(&"Ferris".to_string())); // &String → &String

    // 😱 No coercion here! Uncomment to see an errors.
    // These calls fail because bad expects a &String, but "Ferris" is a &str.
    // bad(&"Ferris") passes a &&str, which cannot be coerced to &String.
    // println!("{:?}", bad(&"Ferris")); // expected reference `&String` found reference `&&'static str`

    // bad("Ferris") passes a &str, which also cannot be coerced to &String.
    // println!("{:?}", bad("Ferris")); // expected reference `&String` found reference `&'static str`
}

#[derive(Debug)]
struct Foo<T> {
    bar: T,
}

impl<T> Deref for Foo<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.bar
    }
}

fn deref_example() {
    let foo_var = Foo { bar: "bar" };

    // Without Deref.
    println!("{:?}", foo_var.bar);

    // With Deref.
    println!("{:?}", *foo_var);

    // Same thing.
    assert_eq!(*foo_var, foo_var.bar);
}

fn foo(bar: &str) {
    println!("{bar}");
}

fn deref_auto_coercions_example() {
    // Because String implements Deref<Target=str>.
    let owned_hello = "Hello".to_string();

    // So this work because &String → &str
    foo(&owned_hello);
    foo(&&owned_hello);
    foo(&*&owned_hello); // Why god why? Stop!

    // Or even this because &Rc<String> → &String → &str
    foo(&std::rc::Rc::new(owned_hello));
}

struct Foo2;

impl Foo2 {
    fn bar(&self) {
        println!("FooBar");
    }
}

fn deref_method() {
    // Same thing.
    Foo2.bar();
    (&Foo2).bar();
    (&&Foo2).bar();
    (&&&Foo2).bar();
}

fn main() {
    good_and_bad_example();

    deref_example();

    deref_auto_coercions_example();

    deref_method();
}
