use std::error::Error;

// from here
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// run
// cargo run --example return_code
#[warn(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
