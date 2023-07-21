#[warn(unused_variables)]
fn main() {
    
    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    for i in &v {
        println!("{}",i)

    }

    //Multi Arrays
    let _a:[i32; 4] = [1,2,3,4];
    let _b:[i32; 4] = [5,6,7,8];
    let _c: [[i32; 4]; 2] = [_a,_b];

    println!("{:?}",_c);

    let _d=_c[1][0];

    println!("d: {}",_d);


}
