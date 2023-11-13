fn main() {
    //  👇 Mutable so we change the value later.
    let mut count = 0;

    // This .. 👇 mean range i from 0 to 7.
    for _i in 0..8 { // _i mean we won't use i
        count += 1;
    }

    println!("count = {count}");

    // This .. 👇 mean range i from 0 to 8.
    for i in 0..=8 {
        println!("i: {0}", i);
        count += i;
    }

    println!("count = {count}", count = count);

    // 👇 This is how we loop element (e).
    for e in ["a","b","c"] {
        println!("{e}");
    }

    //  👇 This is index (i) can be use by 👇 call enumerate fn.
    for (i, e) in ["a","b","c"].iter().enumerate() {
        println!("{i} = {e}");
    }

    // while
    while count < 50 {
        println!("count = {0}", count);
        count += 1;
    }

    println!("count = {0}", count);

    // loop
    loop {
        count += 1;
        if count >= 100 {
            println!("count: {0}", count);
            break;
        }
    }

    println!("count = {}", count);

    // loop and break
    'outer: loop {
        count += 1;

        // Break at 200
        if count >= 200 {
            // Never reach here because 👇.
            break;
        } else {
            // Inner loop
            loop {
                count += 1;
                // Because this will break first.
                if count >= 150 {
                    break 'outer;
                }
            }
        }
    }

    println!("count = {}", count);
}
