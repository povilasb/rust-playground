use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub fn read_numbers<T>() -> Vec<T>
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

pub fn readln() -> io::Result<String> {
    let mut ln = String::new();
    io::stdin().read_line(&mut ln)?;
    Ok(String::from(ln.trim()))
}
