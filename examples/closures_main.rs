fn main() {
    println!("Hello from an example!");

    let mut closures = [|x: i32| x * x, |x| x + 1];
    for closure in closures.iter_mut() {
        println!("{}", (*closure)(10));
    }

    let mut vec:Vec<i32> = Vec::new();
    let mut vec_multi:Vec<i32> = Vec::new();

    // https://www.programiz.com/rust/hashset
    let _colors: &str = {"Green"; "Yellow"; "Red"};
    println!("{:?}",&_colors);


}
