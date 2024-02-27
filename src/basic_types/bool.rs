pub fn run() {
    let f: bool = false;

    if !f {
        println!("Success!");
    }

    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
} 
