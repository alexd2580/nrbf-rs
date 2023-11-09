use std::{io, fs::File};

use nrbf_rs::parse_nrbf;

fn main() -> Result<(), io::Error> {
    let mut stream = File::open("examples/batim.dump")?;
    println!("{}", parse_nrbf(&mut stream));
    Ok(())
}
