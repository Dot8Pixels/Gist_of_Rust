use std::ops::Deref;

trait Human {
    fn name(&self) -> String;
}

trait Learner: Human {
    fn is_enjoy(&self) -> bool;
    fn increase_power(&mut self, amount: u8);
}

trait Coder {
    fn get_language(&self) -> &str;
}

trait Rustaceans: Coder + Learner {
    fn get_blog(&self) -> String;
}

#[derive(Debug, Default)]
struct Me {
    language: String,
    power: u8,
}

impl Me {
    // We derived Default so we need to impl this fn default().
    fn default() -> Self {
        Me {
            power: 0u8,
            language: "🦀 Rust".to_owned(),
        }
    }

    // Here's getter/setter.
    fn set_power(&mut self, amount: u8) {
        self.power = amount
    }
}

impl Human for Me {
    fn name(&self) -> String {
        "katopz".to_owned()
    }
}
impl Learner for Me {
    fn is_enjoy(&self) -> bool {
        true
    }

    fn increase_power(&mut self, amount: u8) {
        self.set_power(self.power + amount);
    }
}
impl Coder for Me {
    fn get_language(&self) -> &str {
        // We need to 👇 deref so👆 we can return &str.
        self.language.deref()
    }
}
impl Rustaceans for Me {
    fn get_blog(&self) -> String {
        "https://katopz.medium.com/".to_owned()
    }
}

struct You {}

// You are just ordinary Human.
impl Human for You {
    fn name(&self) -> String {
        "foo".to_owned()
    }
}

// This👇 T = Type mean this fn will accept Learner Type = Generic Bounds.
fn learn<T: Learner>(t: &mut T) {
    // And can be reuse here 👆.
    t.increase_power(9u8);
}

// We can compose type 👇 with this 👇 = Multiple bounds.
fn join_hackathon<T: Human + Learner>(t: &mut T, amount: u8) {
    t.increase_power(amount);
}

// Or use compose traits as parameters like this
fn enjoy_rust(t: &mut (impl Learner + Coder)) {
    t.increase_power(11u8);
}

// Or dynamic like this
fn blog_rust(t: &mut dyn Rustaceans) {
    t.increase_power(12u8);
}

fn main() {
    let mut me: Me = Me::default();

    // Learn lonely
    learn(&mut me);
    println!("1️⃣ {:?}", me);

    // Join hackathon
    join_hackathon(&mut me, 10u8);
    println!("2️⃣ {:?}", me);

    // 😱 Uncomment below to see `the trait bound `You: Learner` is not satisfied`.
    // join_hackathon(&mut You {}, 100u8);

    // Enjoy!
    enjoy_rust(&mut me);
    println!("3️⃣ {:?}, enjoy {}", me, Coder::get_language(&me));

    // We can do anything. Yeah!
    blog_rust(&mut me);
    println!("4️⃣ {:?}, blog {}", me, Rustaceans::get_blog(&me));
}
