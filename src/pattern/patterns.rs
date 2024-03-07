pub fn run() {
    match_number(4);
    mathch_and_hold_value();
    excercise();
    match_guard();
    remaining();
    mutable();
}

fn match_number(n: i32) {
    match n {
        // Match a single value
        1 => println!("One!"),
        // Fill in the blank with `|`, DON'T use `..` or `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // Match an inclusive range
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

fn mathch_and_hold_value() {
    struct Point {
        x: i32,
        y: i32,
    }
    // Fill in the blank to let p match the second arm
    let p = Point { x: 5, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Second arm
        Point {
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// Fix the errors

fn excercise() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello {
            id: newid @ (10 | 11 | 12),
        } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// Fill in the blank to make the code work, `split` MUST be used
fn match_guard() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if (x < split) => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

// Fill the blank to make the code work
fn remaining() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

// FIX the error with least changing
// DON'T remove any code line
fn mutable() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }
}
