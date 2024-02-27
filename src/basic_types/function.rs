pub fn run() {
    basic();
    print_function();
    excercise();
}

fn basic() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
fn print_function() {
    print();

    fn print() -> () {
        println!("Success!");
    }
}

// Solve it in two ways
// DON'T let `println!` work
fn excercise() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };
    println!("{}", _v);

    println!("Exercise Failed if printing out this line!");
}


