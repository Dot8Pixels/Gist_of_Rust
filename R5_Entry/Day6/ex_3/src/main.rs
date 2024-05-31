// Define a trait called `Combiner` that takes three generic type parameters: A, B, and C.
trait Combiner<A, B, C> {
    // Declare an associated function `combine`.
    fn combine(a: &A, b: &B) -> C;
}

// String + String = String
struct StringCombiner;

impl Combiner<String, String, String> for StringCombiner {
    fn combine(a: &String, b: &String) -> String {
        format!("{}{}", a, b)
    }
}

// i32 + i32 = i32
struct NumberCombiner;

impl Combiner<i32, i32, i32> for NumberCombiner {
    fn combine(a: &i32, b: &i32) -> i32 {
        *a + *b
    }
}

fn main() {
    // Combine string.
    let str1: String = String::from("Hello, ");
    let str2: String = String::from("world!");

    let str_result: String = StringCombiner::combine(&str1, &str2);
    println!("String result: {}", str_result);

    // Combine number
    let num1: i32 = 5;
    let num2: i32 = 10;

    let num_result: i32 = NumberCombiner::combine(&num1, &num2);
    println!("Number result: {}", num_result);
}
