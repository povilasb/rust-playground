#[cfg(test)] #[macro_use] extern crate hamcrest;

mod tree;

fn main() {
    let mut t = tree::Tree::new();
    t.append(Some(2), Some(3), 1);
    println!("Hello, world!");
}
