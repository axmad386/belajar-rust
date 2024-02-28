pub fn run() {
    basic();
    borrow();
    slice_from_array();
    string_slice();
    string_slice2();
}

// Fix the errors, DON'T add new lines!
// Here, both [i32] and str are slice types, but directly using it will cause errors. You have to use the reference of the slice instead: &[i32], &str
fn basic() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];

    let _s2: &str = "hello, world" as &str;

    println!("Success!");
}


fn borrow() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16); // pointer size+ char size = 8 + 4 * 2;

    println!("Success!");
}


fn slice_from_array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32]= &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}


fn string_slice() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

// Fix errors
fn string_slice2() {
    let s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter = first_letter(&s);

    // s.clear(); // error!

    println!("the first letter is: {}", letter);
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
