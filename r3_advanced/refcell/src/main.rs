use std::cell::RefCell;
use std::ops::DerefMut;
struct Foo {
    i: i32,
}

struct Bar<'b> {
    // store the data in a RefCell for interior mutability
    f: &'b RefCell<Foo>,
}

impl<'a: 'b, 'b> Bar<'b> {
    // Return a RefMut smart pointer instead of mutable ref, but hide the implementation,
    // just exposing it as something that can be mutably dereferenced as a Foo
    fn func(&'a self) -> impl DerefMut<Target = Foo> + 'b {
        self.f.borrow_mut()
    }
}

fn without_refcell() {
    let mut seen = vec![];
    let items = [vec![1i32, 2], vec![3], vec![1]];

    let a: Vec<_> = items
        .iter()
        .flat_map(|inner_numbers| inner_numbers.iter())
        .filter_map(move |&number| {
            if !seen.contains(&number) {
                seen.push(number);
                Some(number)
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", a);
}

fn with_refcell() {
    let seen = vec![];
    let items = [vec![1i32, 2], vec![3], vec![1]];

    let seen_cell = RefCell::new(seen);

    let a: Vec<_> = items
        .iter()
        .flat_map(|inner_numbers| {
            inner_numbers.iter().filter_map(|&number| {
                let mut borrowed = seen_cell.borrow_mut();

                if !borrowed.contains(&number) {
                    borrowed.push(number);
                    Some(number)
                } else {
                    None
                }
            })
        })
        .collect();

    println!("{:?}", a);
}

fn main() {
    without_refcell();

    with_refcell();

    let foo_var = RefCell::new(Foo { i: 1 });
    let bar_var = Bar { f: &foo_var };

    let mut f = bar_var.func();
    f.i = 3;
}
