// gunakan mut untuk membuat variable mutable
pub fn run(){
    let mut x: i32 = 5;
    assert_eq!(x,5);

    x-=2;
    assert_eq!(x,3);
    println!("Success!");
}
