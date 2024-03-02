pub fn run(){
    basic();
    expression();
    labels();
}

// Fill in the blanks
fn basic() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

// Fill in the blank
fn expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

// Fill in the blank
fn labels() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}
