fn main() {
    // Define immutable variable.
    let count = 0;

    // {} mean param_0.
    println!("count = {}", count);

    // Define mutable variable.
    let mut count = 1;

    // So we can change it.
    count += 1;

    // {0} mean param_0.
    // {1} mean param_1.
    //         ╭────────────────╮
    println!("{0} = {1:#?}", "count", count);
    //               ╰──────────────────╯
    // # mean pretty print.
    // ? mean debug.

    // Let's make some condition.
    if count == 2 {
        // String literal {count} mean variable count for Display.
        println!("count = {count}");
    }

    // Assert that count is equal 2.
    assert_eq!(count, 2);

    // As base 16 hexadecimal by adding 👇.
    println!("count = {count} = 0x{count:x}");
}
