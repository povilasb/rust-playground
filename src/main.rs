#[cfg(test)] #[macro_use] extern crate hamcrest;

mod tree;
use tree::Tree;

fn main() {
    let mut t = Tree::new();
    t.append(Some(2), Some(3), 1);
    t.append(Some(4), None, 2);
    t.append(Some(5), None, 3);
    println!("{}", t.to_string());
}
