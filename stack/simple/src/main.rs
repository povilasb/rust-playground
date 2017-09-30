use std::io;
use std::str::FromStr;
use std::fmt::Debug;
use std::cmp::Ord;
use std::clone::Clone;

struct Stack<T> {
    items: Vec<T>,
}

impl <T: Ord + Clone> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) {
        self.items.pop();
    }

    fn max(&self) -> T {
        (*self.items.iter().max().unwrap()).clone()
    }
}

fn main() {
    let query_count = read_numbers::<usize>()[0];
    let mut nums = Stack::new();

    for _ in 0..query_count {
        let query = read_numbers::<usize>();
        match query[0] {
            1 => nums.push(query[1]),
            2 => nums.pop(),
            3 => println!("{}", nums.max()),
            _ => panic!("unknown query"),
        };
    }
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
