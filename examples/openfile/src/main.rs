use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let content = read_file("foo.txt")?;
    println!("content: {:?}", content);
    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer)?;
    Ok(buffer)

}
