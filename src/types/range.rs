pub fn run() {
    let mut sum = 0;
    for i in -3..2 { // 2 akan diexclude. Jadi iterasi hanya sampai 1
        sum += i
    }

    assert_eq!(sum, -5);

    let mut sum = 0;
    for i in -3..=2 { // 2 akan include. Jadi iterasi sampai 2
        sum += i
    }

    assert_eq!(sum, -3);

    for c in 'a'..='z' {
        println!("{} => {}",c , c as u8);
    }
}
