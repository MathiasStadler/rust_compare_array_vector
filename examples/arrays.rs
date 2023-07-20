fn main() {
    println!("arrays");
    // from here
    // https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/arrays-vectors-and-slices.html

    let _a = [1, 2, 3];
    let a = [0; 5];

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e);
    }
    println!("The fourth number is: {}", a[4]);

    let names: [&str; 4] = ["Graydon", "Brian", "Niko", "Mathias"]; // names: [&str; 3]

    println!("The fourth name is: {}", names[3]);

    println!("Vector: A vector is a dynamic or \"growable\" array, implemented as the standard library type Vec<T>");
    let _v: Vec<i32> = vec![1, 2, 3]; // v: Vec<i32>

    let mut nums: Vec<i32> = vec![1, 2, 3]; // mut nums: Vec<i32>

    // add to Vec
    nums.push(4);
    // add to Vec
    nums.push(5);

    for e in nums.iter() {
        println!("nums {}", e); // Prints 1, 2, 3
    }
    

    println!("The length of nums is now {}", nums.len()); // Prints 4

    let list: &[i32] = &vec![1, 3, 4, 17, 81];

    for (i, el) in list.iter().enumerate() {
        println!("current index {} element  {}", i,el);
        // println!("The current index is {}", i);
    }

    for (i, el) in nums.iter().enumerate() {
        println!("nums: current index {} get value  {}", i,el);
        // println!("The current index is {}", i);
    }

}
