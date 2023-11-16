fn main() {
    // Create new `array` of `&str` and `vec`.
    let array_of_foo = ["foo", "bar"]; // Array of &str.
    let mut vec_of_foo = vec!["foo", "bar"]; // Say hi to vec! macro.

    println!("array_of_foo: {array_of_foo:#?}");
    println!("vec_of_foo: {vec_of_foo:#?}");

    // The different?
    vec_of_foo.push("baz"); // You can push more to Vec

    // 😱 Uncomment to see an error "no method named `push` found for array `[&str; 2]`".
    // FYI: `[&str; 2]` mean fixed array of &str usize 2.
    // 👍 Anyway fixed size is good for memory management, so don't hate it!
    // array_of_foo.push("baz"); // You can't to fixed Array [&str; 2]

    // 1️⃣ Back to Vec, Let's iterate them.
    let hello_vec = vec_of_foo
        .iter() // Must `iter()` before you can map, filter,...
        .map(|e| format!("hello {e}")) // Say hi to `closure` |e| aka (e)=> in js.
        .collect::<Vec<_>>(); // `collect` inferred type from iterate.
        //             👆 `_` is inferred type (let compiler desire).

    println!("hello_vec: {hello_vec:#?}");

    // 2️⃣ Do it again but with index.
    let indexed_vec = vec_of_foo
        .iter()
        .enumerate() // To access index we need `enumerate`.
        .map(|(i, e)| (i, e)) // Say hi to `Tuple` type.
        .collect::<Vec<(usize, &&str)>>(); // i is `usize`, e is &&str.

    println!("indexed_vec: {indexed_vec:#?}");

    // 3️⃣ Do it again but `into_iter`.
    let into_iter_indexed_vec = vec_of_foo
        .into_iter() // `into_iter` instead of `iter` for `deref` (Wait what?).
        .enumerate()
        .map(|(i, e)| (i, e))
        .collect::<Vec<(usize, &str)>>(); // e is just &str not &&str.
                                          // Or just `<Vec<_>>` if you lazy.

    println!("into_iter_indexed_vec: {into_iter_indexed_vec:#?}");

    // `into_iter` is handy to pass value without borrow
    // but it can be problematic sometime if it has been borrowed by 1️⃣ and 2️⃣.

    // 😱 Uncomment this to see an error.
    // assert_eq!(
    //     indexed_vec.first().unwrap().1,
    //     &into_iter_indexed_vec.first().unwrap().1
    // );
}
