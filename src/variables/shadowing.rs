pub fn run(){
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".

    let a: i32;
    a = 7;
    // Shadowing and re-binding
    let mut a = a; 
    a += 3;
    assert_eq!(a,10);

    let _y = 4;
    // Shadowing
    let _y: &str = "I can also be bound to text!"; 

    println!("Success!");
}
