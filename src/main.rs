#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

mod tree;
use tree::Tree;

fn main() {
    let node_count = read_numbers::<usize>()[0];

    let mut t = Tree::new();
    for i in 0..node_count {
        let indexes = read_indexes();
        if !empty_node(&indexes) {
            t.append(indexes[0], indexes[1], i + 1);
        }
    }

    let swap_count = read_numbers::<usize>()[0];
    let mut swap_at: Vec<usize> = Vec::new();
    for _ in 0..swap_count {
        swap_at.push(read_numbers::<usize>()[0]);
    }

    for depth in swap_at {
        swap_children_at_divisable(&mut t, depth);
        println!("{}", t.to_string());
    }
}

fn swap_children_at_divisable(t: &mut Tree, depth: usize) {
    for d in 1..t.depth() {
        if d % depth == 0 {
            swap_children_at_depth(t, d);
        }
    }
}

fn swap_children_at_depth(t: &mut Tree, depth: usize) {
    let nodes = t.nodes_at_depth(depth);
    for n in nodes {
        n.swap_children();
    }
}

fn empty_node(indexes: &Vec<Option<u64>>) -> bool {
    indexes.iter().all(|n| n.is_none())
}

fn read_indexes() -> Vec<Option<u64>> {
    read_numbers::<i64>().iter()
        .map(|n: &i64| match *n {
            n if n < 0i64 => None,
            _ => Some(*n as u64),
        })
        .collect()
}

fn read_numbers<T>() -> Vec<T>
where
    T: FromStr,
    T::Err: Debug
{
    match readln() {
        Ok(ln) => {
            ln.split_whitespace()
                .map(|s| s.parse::<T>().unwrap())
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
