use std::io::{self};
use std::cmp;

struct Operation {
    a: usize,
    b: usize,
    k: u64,
}

impl Operation {
    fn new(a: usize, b: usize, k: u64) -> Operation {
        Operation{a: a - 1, b: b - 1, k: k}
    }
}

fn main() {
    let nm = read_numbers();
    let n = nm[0]; // array size
    let m = nm[1]; // operation count

    let mut ops: Vec<Operation> = Vec::new();
    for _ in 0..m {
        let abk = read_numbers();
        ops.push(Operation::new(abk[0], abk[1], abk[2] as u64));
    }

    let mut arr = vec![0; n + 1];

    for op in ops {
        arr[op.a] += op.k;
        arr[op.b + 1] -= op.k;
    }

    let mut max = 0;
    let mut sum = 0;
    for n in arr {
        sum += n;
        max = cmp::max(sum, max);
    }
    println!("{}", max);
}

fn read_numbers() -> Vec<usize> {
    match readln() {
        Ok(ln) => {
            ln.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
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
