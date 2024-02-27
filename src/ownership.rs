pub fn run() {
    basic();
    basic2();
    basic3();
    basic4();
    basic5();
    take();
    give();
    clone();
    copy();
    mutability();
    mutability2();
    partial_move();
    partial_move_exercise();
    partial_move_exercise2();
}

fn basic() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}
fn basic2() {
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}", x, y);
}

fn basic3() {
    let x = "Hello World";
    let y = x;
    println!("{}, {}", x, y);
}
fn basic4() {
    let x = &String::from("Hello World");
    let y = x;
    println!("{}, {}", x, y);
}
fn basic5() {
    let x = String::from("Hello World");
    let y = x.as_str();
    println!("{}, {}", x, y);
}
fn take() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    // println!("{}", s1); --> this will fail

    println!("{}", s2);
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }
}
fn give() {
    let s = give_ownership();
    println!("{}", s);
    fn give_ownership() -> String {
        let s = String::from("Hello World");
        let _s = s.clone().into_bytes(); // convert String to Vec
        s
    }
}
// Fix the error without removing any code
fn clone() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
    fn print_str(s: String) {
        println!("{}", s)
    }
}
fn copy() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
fn mutability() {
    let s = String::from("Hello");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success");
}
fn mutability2() {
    let x = Box::new(5);

    let mut y = Box::new(5);

    assert_eq!(*x, 5);

    *y = 4;

    assert_eq!(*x, 5);
    assert_eq!(*y, 4);

    println!("Success!");
}
fn partial_move(){
    struct Person{
        name: String,
        age: Box<u8>
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20)
    };

    let Person {name, ref age} = person;
    println!("The person's name is {} and age is {}", name, age);

    // person cannot be used, but person.age can be used as it not moved
    println!("THe person age from person struct is {}", person.age);
}

 fn partial_move_exercise(){
     let t = (String::from("hello"), String::from("world"));
     let _s = t.0;

     println!("{:?}", t.1);
 }
fn partial_move_exercise2(){
    let t = (String::from("hello"), String::from("world"));
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
