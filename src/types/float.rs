pub fn run(){
    float();
    sum();

}
// Fill the blank to make it work
fn float() {
    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn sum() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
    assert!(0.1 as f32+0.2 as f32==0.3 as f32);

    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
