pub fn run() {
    println!("===String===");
    let text = "Hello World";
    let text_string = text.to_string();
    println!("{}", text_string);

    let mut hello = String::from("Hello");
    hello.push_str(" World");
    println!("{}", hello);

    let hello = String::from("Hello");
    let world = String::from("World");
    let hello_world = format!("{} {}", hello, world);
    println!("{}", hello_world);
}
