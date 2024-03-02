pub fn run() {
    basic();
    continue_break();
}

// Fill in the blanks to make the last println! work !
fn basic() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

// Fill in the blank
fn continue_break() {
    let mut n = 0;
    for _i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");

    // Fill in the blanks
    let mut n = 0;
    for _i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
