use std::fs::File;
use std::io::{self, Write};
use std::fmt::Display;

pub fn write_to_file(fname: &str, num: u64) -> io::Result<()> {
    let mut f = File::create(fname)?;
    write!(&mut f, "{}", num);
    Ok(())
}

pub fn writeln_to<T: Display>(fname: &str, data: T) -> io::Result<()> {
    let mut f = File::create(fname)?;
    write!(&mut f, "{}\n", data);
    Ok(())
}
