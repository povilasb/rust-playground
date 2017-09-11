#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

struct MinHeap {
    items: Vec<i64>,
}

impl MinHeap {
    fn new() -> MinHeap {
        MinHeap {
            items: Vec::new(),
        }
    }

    fn push(&mut self, item: i64) {
        self.items.push(item);
        let item_count = self.items.len();
        if item_count > 1 {
            reorder(&mut self.items, item_count - 1);
        }
    }

    fn remove(&mut self, item: i64) {
        match self.items.iter().enumerate().find(|&(_, val)| *val == item) {
            Some((pos, _)) => self.items.remove(pos),
            None => 0,
        };
    }
}

fn reorder(items: &mut Vec<i64>, item_pos: usize) {
    match parent_pos(item_pos) {
        Some(parent_pos) => {
            if items[item_pos] < items[parent_pos] {
                items.swap(item_pos, parent_pos);
                reorder(items, parent_pos);
            }
        },
        None => (),
    }
}

fn parent_pos(child_pos: usize) -> Option<usize> {
    match child_pos {
        0 => None,
        _ => Some((child_pos - 1) / 2),
    }
}

type Query = Vec<i64>;

fn main() {
    let query_count = read_numbers::<usize>()[0];

    let mut heap: MinHeap = MinHeap::new();
    for _ in 0..query_count {
        let query = read_numbers::<i64>();
        process_query(&mut heap, &query);
    }
}

fn process_query(heap: &mut MinHeap, query: &Query) {
    match query[0] {
        1 => heap.push(query[1]),
        3 => println!("{}", heap.items.iter().min().unwrap()),
        2 => heap.remove(query[1]),
        _ => panic!("Unknown query `{}`", query[0]),
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

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    // Trying to emulate nesting what "stainless" lib already does.
    mod heap {
        use super::*;

        #[test]
        fn push_when_heap_is_empty_only_specified_item_is_appended() {
            let mut heap = MinHeap::new();

            heap.push(20);

            assert_that!(heap.items[0], is(equal_to(20)));
        }

        #[test]
        fn push_when_heap_has_bigger_item_it_is_reordered() {
            let mut heap = MinHeap::new();
            heap.push(20);

            heap.push(10);

            assert_that!(&heap.items, contains(vec![10, 20]).in_order());
        }

        #[test]
        fn push_swaps_smaller_items_recursively() {
            let mut heap = MinHeap::new();
            heap.push(10);
            heap.push(15);
            heap.push(20);

            heap.push(7);

            assert_that!(&heap.items, contains(vec![7, 10, 20, 15]).in_order());
        }
    }

    #[test]
    fn parent_pos_returns_parent_node_position_by_child_node_when_such_exists() {
        let pos = parent_pos(3).unwrap();

        assert_that!(pos, is(equal_to(1)));
    }

    #[test]
    fn parent_pos_returns_none_when_child_pos_is_0() {
        let pos = parent_pos(0);

        assert_that!(pos, is(none::<usize>()));
    }
}
