#[warn(unused_variables)]
fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    for i in &v {
        println!("{}", i)
    }

    //Multi Arrays
    let _a: [i32; 4] = [1, 2, 3, 4];
    let _b: [i32; 4] = [5, 6, 7, 8];
    let mut _c: [[i32; 4]; 2] = [_a, _b];

    println!("{:?}", _c);

    let _d = _c[1][0];

    println!("d: {}", _d);

    
    
    // not work array has fix size
    //_c[1][4] = 5;

    // multi vector
    let mut vec1 = Vec::new();
    vec1.push(1);
    let mut vec2 = Vec::new();
    vec2.push(2);
    let mut vec3 = Vec::new();
    vec3.push(vec1);
    vec3.push(vec2);

    println!("vec3 => {:?}",vec3);
}

// Methods for Array Initialization in Rust
// https://www.joshmcguigan.com/blog/array-initialization-rust/