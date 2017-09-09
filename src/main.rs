#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

struct Queue<T: Copy> {
    values: Vec<T>,
}

impl <T: Copy> Queue<T> {
    fn new(size: usize) -> Queue<T> {
        Queue {
            values: Vec::with_capacity(size),
        }
    }

    fn append(&mut self, value: T) {
        self.values.push(value);
    }

    fn iter(&self) -> QueueIterator<T> {
        QueueIterator {
            queue: self,
            curr_item: 0,
        }
    }

    fn iter_at(&self, start: usize) -> QueueIterator<T> {
        QueueIterator {
            queue: self,
            curr_item: start,
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}


struct QueueIterator<'a, T: Copy + 'a> {
    queue: &'a Queue<T>,
    curr_item: usize,
}

impl <'a, T: Copy> Iterator for QueueIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_item > self.queue.values.len() - 1 {
            self.curr_item = 0;
        }
        let res = Some(self.queue.values[self.curr_item]);
        self.curr_item += 1;
        res
    }
}

fn main() {
    let pump_count = read_numbers::<usize>()[0];
    let mut pumps: Queue<(i64, i64)> = Queue::new(pump_count);

    let mut tank_volume = 0i64;
    for _ in 0..pump_count {
        let stop = read_numbers::<i64>();
        pumps.append((stop[0], stop[1]));
    }
    println!("{}", start_trip_at(&pumps));
}

fn start_trip_at(pumps: &Queue<(i64, i64)>) -> usize {
    for station in 0..pumps.len() {
        match gas_after_trip_at(&pumps, station) {
            Some(gas) => return station,
            None => (),
        }
    }
    0
}

fn gas_after_trip_at(pumps: &Queue<(i64, i64)>, station: usize) -> Option<i64> {
    let mut gas = 0i64;
    for p in pumps.iter_at(station).take(pumps.len()) {
        gas = gas + p.0 - p.1;
        if gas < 0 {
            return None;
        }
    }
    Some(gas)
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
    mod queue {
        use super::*;

    }
}
