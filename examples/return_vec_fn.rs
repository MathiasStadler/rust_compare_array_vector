fn get_new_vector() -> Vec<String> {
    // Return a vector from a function.
    vec!["a".to_string(), "b".to_string()]
}

fn main() {
    let result = get_new_vector();
    println!("{:?}", result);

    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    println!("{:?}", data);
    &data[0][1] = 9;

    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    //assert_eq!(flattened, &[1, 2, 3, 4, 5, 6]);

    println!("{:?}", flattened);
}
