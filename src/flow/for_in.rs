pub fn run(){
    basic();
    excercise();
    iterate();
}

fn basic() {
    for n in 1..100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 

// Fix the errors without adding or removing lines
fn excercise() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        println!("{}", n);
    }
    
    println!("{:?}", numbers);
} 
fn iterate() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}
