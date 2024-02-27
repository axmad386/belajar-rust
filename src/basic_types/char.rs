use std::mem::size_of_val;
pub fn run() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");


    let c1 = "中";
    print_char(c1);

}
fn print_char(c : &str) {
    println!("{}", c);
}
