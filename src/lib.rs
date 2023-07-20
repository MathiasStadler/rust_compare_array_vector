pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn get_array() -> () {
    
    //https://doc.rust-lang.org/rust-by-example/primitives/array.html
    [1, 2, 3, 4, 5];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
