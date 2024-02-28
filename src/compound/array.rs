pub fn run(){
    basic();
    basic2();
    basic3();
    basic4();
    indexing();
    outbonds();
}


fn basic() {
    // Fill the blank with proper array type
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}

fn basic2() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
    assert!(std::mem::size_of_val(&arr0) == 12);

    println!("Success!");
}

fn basic3() {
    // Fill the blank
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

// all elements in array must be of the same type
fn basic4() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}

//indexing start at 0
fn indexing(){

    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}


// Fix the error
fn outbonds() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let _name0 = names.get(2).unwrap(); // compiled but panicked

    // But indexing is not safe
    // let _name1 = &names[2]; --> error : index out of bounds

    println!("Success!");
}
