use std::io::{self};

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
    println!("{}", "ops constructed");

    let mut arr = vec![0; n];

    for op in ops {
        apply_op(&mut arr, &op);
    }
    println!("{}", arr.iter().max().unwrap());
}

fn apply_op(arr: &mut Vec<u64>, op: &Operation) {
    for i in op.a..op.b + 1 {
        arr[i] += op.k;
    }
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
