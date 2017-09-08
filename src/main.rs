#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::io;

mod tree;
use tree::Tree;

fn main() {
    let mut t = Tree::new();
    t.append(Some(2), Some(3), 1);
    t.append(None, Some(4), 2);
    t.append(None, Some(5), 3);

    swap_children_at_divisable(&mut t, 2);
}

fn swap_children_at_divisable(t: &mut Tree, depth: usize) {
    for d in 1..t.depth() {
        if d % depth == 0 {
            swap_children_at_depth(t, d);
            println!("{}", t.to_string());
        }
    }
}

fn swap_children_at_depth(t: &mut Tree, depth: usize) {
    let nodes = t.nodes_at_depth(depth);
    for n in nodes {
        n.swap_children();
    }
}

fn read_indexes() -> Vec<Option<u64>> {
    read_numbers().iter()
        .map(|n: &i64| match *n {
            n if n < 0i64 => None,
            _ => Some(*n as u64),
        })
        .collect()
}

fn read_numbers() -> Vec<i64> {
    match readln() {
        Ok(ln) => {
            ln.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        },
        Err(_) => Vec::new(),
    }
}

fn readln() -> io::Result<String> {
    let mut ln = String::new();
    io::stdin().read_line(&mut ln)?;
    Ok(String::from(ln.trim()))
}
