use std::string::ToString;
use std::cmp::PartialOrd;
use std::clone::Clone;

mod stdio;
use stdio::{read_numbers, readln};

fn main() {
    readln(); // skip
    let arr = read_numbers::<i64>();
    let (left, equal, right) = partition(&arr[..], &arr[0]);
    println!("{} {} {}", arr_to_str(&left), arr_to_str(&equal),
             arr_to_str(&right));
}

fn arr_to_str<T: ToString>(arr: &Vec<T>) -> String {
    arr.iter()
        .fold("".to_string(), |acc, ref num| acc + &num.to_string()[..] + " ")
        .trim()
        .to_string()
}

fn partition<T>(arr: &[T], pivot: &T) -> (Vec<T>, Vec<T>, Vec<T>)
    where T: PartialOrd + Clone
{
    let mut left: Vec<T> = Vec::new();
    let mut equal: Vec<T> = Vec::new();
    let mut right: Vec<T> = Vec::new();
    for num in arr {
        if num < pivot {
            left.push(num.clone());
        } else if num == pivot {
            equal.push(num.clone());
        } else {
            right.push(num.clone());
        }
    }
    (left, equal, right)
}
