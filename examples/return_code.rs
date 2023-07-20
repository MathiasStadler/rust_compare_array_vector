use std::error::Error;
use std::fs::File;
// from here
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}