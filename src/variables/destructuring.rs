pub fn run(){
    basic_destructuring();
    destructuring_assignment();
}

fn basic_destructuring(){
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
fn destructuring_assignment() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1,5, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
} 
