pub fn run() {
    let mut sum = 0;
    for i in -3..2 {
        // 2 akan diexclude. Jadi iterasi hanya sampai 1
        sum += i
    }

    assert_eq!(sum, -5);

    let mut sum = 0;
    for i in -3..=2 {
        // 2 akan include. Jadi iterasi sampai 2
        sum += i
    }

    assert_eq!(sum, -3);

    for c in 'a'..='z' {
        println!("{} => {}", c, c as u8);
    }

    // Fill the blanks
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
