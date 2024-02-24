pub fn run() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is: {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    define_z();
}

fn define_z() {
    let z = "hello";
    println!("{} world", z);
}
