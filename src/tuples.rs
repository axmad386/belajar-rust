pub fn run() {
    println!("===tuples===");
    fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
        let (a, b) = pair;
        (b, a)
    }

    let pair = (2, true);
    println!("pair is {:?}", pair);
    println!("first item of pair is {}", pair.0);
    println!("second item of pair is {}", pair.1);
    println!("the reversed pair is {:?}", reverse_pair(pair));

    #[derive(Debug)]
    struct Matrix (f32, f32, f32, f32);
    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{:?}", matrix);
}
