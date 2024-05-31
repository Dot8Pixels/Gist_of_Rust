// We previously use a lot of "count", let's DRY it as a constant.
const COUNT: &str = "count"; // Say hi to referenced string slice &str

// And maybe we want static footgun 💥 that can mutate.
static mut TOTAL: u32 = 0;

// Define "add" as a function
fn add(a: i32, b: i32) -> i32 {
    // i32 = integer 32
    a + b // This mean return a + b, hence no semicolon ;
}

fn main() {
    // We can use assert instead of assert_eq for test.
    assert!(add(1, 2) == 3);
    assert_eq!(add(1,2),3);

    // Try use COUNT with format!
    let result = format!("{COUNT} = {}", add(1, 9));
    println!("{result}");

    // We will need unsafe to mutate static (fyi: bad practice).
    unsafe {
        // Try mutate and 👇 cast i32 to u32 (unsigned integer 32)
        TOTAL = add(3, 4) as u32;

        // Try assert_eq.
        assert_eq!(TOTAL, 7);
    }
}
