trait Human {
    fn name(&self) -> String;
}

trait Learner: Human {
    fn is_enjoy(&self) -> bool;
}

trait Coder {
    fn get_language(&self) -> String;
}

trait Rustaceans: Coder + Learner {
    fn get_blog(&self) -> String;
}

struct Me {}
impl Human for Me {
    fn name(&self) -> String {
        "katopz".to_owned()
    }
}
impl Learner for Me {
    fn is_enjoy(&self) -> bool {
        true
    }
}
impl Coder for Me {
    fn get_language(&self) -> String {
        "rust".to_owned()
    }
}
impl Rustaceans for Me {
    fn get_blog(&self) -> String {
        "https://katopz.medium.com/".to_owned()
    }
}

fn greeting_rustaceans(someone: &dyn Rustaceans) -> String {
    format!(
        "My name is {}, I {} coding in {}, you can visit my blog at {}.",
        someone.name(),
        someone
            .is_enjoy()
            .then(|| "enjoy".to_owned())
            .unwrap_or_else(|| "sad".to_owned()),
        someone.get_language(),
        someone.get_blog(),
    )
}

fn main() {
    println!("{}", greeting_rustaceans(&Me {}));
}
