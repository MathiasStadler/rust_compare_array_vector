// from here
// https://www.programiz.com/rust/iterator

fn main() {
    let numbers = [2, 1, 17, 99, 34, 56];
    
    // iterator
    let numbers_iterator = numbers.iter();

    
    for number in numbers_iterator {
        println!("{}", number);
    }


    let _colors: &str = {
        "Green";
        "Yellow";
        "Red"
    };

    let _colors_iterator = _colors.iter();
}