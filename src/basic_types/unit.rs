use std::mem::size_of_val;
pub fn run() {
    let v: () = ();
    assert_eq!(v, implicitly_ret_unit());
    assert_eq!(v, explicitly_ret_unit());

    assert!(size_of_val(&v) == 0);

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
