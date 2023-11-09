use std::{io, fs::File};

use nrbf_rs::parse_nrbf;

fn main() -> Result<(), io::Error> {
    let mut stream = File::open("examples/dorfromantik.dump")?;
    // parse_nrbf(&mut stream);
    println!("{}", parse_nrbf(&mut stream));
    Ok(())
}
