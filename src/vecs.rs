pub fn run() {
    println!("===Vector===");
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    match numbers.first() {
        Some(v) => println!("{}", v),
        None => panic!("index not found"),
    }

    let mut numbers2 = vec![2, 4, 8];
    for _number in &mut numbers2 {
        *_number *= *_number;
        println!("{}", _number);
    }
    println!("{}", numbers2[0]);
}
