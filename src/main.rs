use std::error::Error;
use std::fs::File;

#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}