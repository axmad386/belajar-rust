pub fn run() {
    basic();
    basic2();
    borrow();
    borrow2();
    borrow3();
    reff();
    rule1();
    rule2();
    rule3();
    nll();
    nll2();
}

fn basic() {
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p);
}

fn basic2() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);
    println!("Success");
}

// Fix error
fn borrow() {
    let s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
    fn borrow_object(_s: &String) {}
}

fn borrow2() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("{}", s);
    fn push_str(s: &mut String) {
        s.push_str("world")
    }
}
fn borrow3() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");
    println!("{}", p);

    println!("Success!");
}

fn reff() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}
fn rule1() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    println!("Success!");
}
fn rule2() {
    let mut s = String::from("hello, ");
    borrow_object(&mut s);
    fn borrow_object(_s: &mut String) {}
}
fn rule3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
    fn borrow_object(_s: &String) {}
}

// Comment one line to make it work
fn nll() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}", r1);
}

fn nll2() {
    let mut s = String::from("hello, ");

    let _r1 = &mut s;
    let _r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    // println!("{}", _r1) -> this will error
}
