use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::str::FromStr;
use std::fmt::Debug;

mod fio;

/// File reader with convenient API.
struct FileInput {
    file: BufReader<File>,
}

impl FileInput {
    /// Opens file for reading. Panics on failure.
    fn open(fname: &str) -> FileInput {
        let file = File::open(fname).expect("Failed to open the file");
        FileInput {
            file: BufReader::new(file),
        }
    }

    /// Reads single line from file. New line symbol is excluded.
    fn readln(&mut self) -> io::Result<String> {
        let mut ln = String::new();
        self.file.read_line(&mut ln)?;
        Ok(ln.trim().to_string())
    }

    fn read_number<T>(&mut self) -> io::Result<T>
    where
        T: FromStr,
        T::Err: Debug
    {
        Ok(self.readln()?.parse::<T>().unwrap())
    }

    /// Tries to read multiple numbers in one file.
    fn read_numbers<T>(&mut self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug
    {
        match self.readln() {
            Ok(ln) => {
                ln.split_whitespace()
                    .map(|s| s.parse::<T>().unwrap())
                    .collect()
            },
            Err(_) => Vec::new(),
        }
    }
}

fn main() {
    let mut fin = FileInput::open("nums.txt");
    println!("{}", fin.read_number::<u32>().unwrap());
    for n in fin.read_numbers::<u64>() {
        println!("{}", n);
    }

    fio::write_to_file("out.txt", 5);
    fio::writeln_to("out.txt", "works:)".to_string());
}
