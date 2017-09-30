use std::io;
use std::cmp::PartialEq;

mod stack;
use stack::Stack;
mod stdio;

#[derive(PartialEq)]
enum Operation {
    Append(String),
    Delete(usize),
    Print(usize),
    Undo(),
}

impl Operation {
    fn from_str(ln: &String) -> io::Result<Operation> {
        let words: Vec<&str> = ln.split_whitespace().collect();
        let opcode = words[0].parse::<u8>().unwrap();
        match opcode {
            1 => Ok(Operation::Append(words[1].to_string())),
            2 => Ok(Operation::Delete(words[1].parse::<usize>().unwrap())),
            3 => Ok(Operation::Print(words[1].parse::<usize>().unwrap())),
            4 => Ok(Operation::Undo()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown opcode")),
        }
    }
}

fn main() {
    let op_count = stdio::read_numbers::<usize>()[0];

    let mut operations: Stack<Operation> = Stack::new();
    let mut curr_str = "".to_string();

    for _ in 0..op_count {
        let op = read_operation().unwrap();
        match op {
            Operation::Append(word) => {
                curr_str.push_str(&word[..]);
                operations.push(Operation::Delete(word.len()));
            },
            Operation::Delete(chars) => {
                let new_size = curr_str.len() - chars;
                operations.push(Operation::Append(
                    curr_str.chars().skip(new_size).collect()));
                curr_str.truncate(new_size);
            }
            Operation::Print(i) => println!("{}", curr_str.chars().nth(i - 1).unwrap()),
            Operation::Undo() => {
                match operations.pop().unwrap() {
                    Operation::Append(word) => curr_str.push_str(&word[..]),
                    Operation::Delete(chars) => {
                        let new_size = curr_str.len() - chars;
                        curr_str.truncate(new_size);
                    },
                    _ => panic!("Unexpected operation during Undo."),
                }
            }
            _ => (),
        }
    }
}

fn read_operation() -> io::Result<Operation> {
    let ln = stdio::readln()?;
    Operation::from_str(&ln)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn from_str_returns_parsed_append_operation() {
            let op = Operation::from_str(&"1 abc".to_string()).unwrap();

            assert!(op == Operation::Append("abc".to_string()));
        }

        #[test]
        fn from_str_returns_parsed_delete_operation() {
            let op = Operation::from_str(&"2 3".to_string()).unwrap();

            assert!(op == Operation::Delete(3));
        }
    }
}
