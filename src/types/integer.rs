pub fn run() {
    integer();
    integer2();
    integer3();
    max_int();
    checked_add();
    multiple_sum();
}
fn integer() {
    let x: i32 = 5;
    let mut y = 5;
    assert_eq!(5, y);

    y = x;

    assert_eq!(5, y);
    assert_eq!(5, x);
    let z: u32 = 10; // Type of z ?
    assert_eq!(10, z);

    println!("Success!");
}

fn integer2() {
    let v: u16 = 38_u8 as u16;
    assert_eq!(38, v);

    println!("Success!");
}
fn integer3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn max_int(){
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

fn checked_add() {
   let v1 = 251_u16 + 8;
   let v2 = u16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}
fn multiple_sum() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1_024 + 255 + 63 + 255);

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
