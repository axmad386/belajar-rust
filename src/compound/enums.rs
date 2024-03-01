pub fn run() {
    basic();
    variant();
    excercise();
    excercise2();
    linked_list();
}

// Fix the errors
#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
#[allow(dead_code)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn basic() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    println!("Success!");
}

fn variant() {
    // Fill in the blank
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg1 = Message::Move { x: 1, y: 1 };
    let msg2 = Message::Write("hello world".to_string()); // Instantiating with "hello, world!"
    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("Success!");
    if let Message::Move { x: a, y: b } = msg1 {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}

fn excercise() {
    // Fill in the blank and fix the errors
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }
}

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn excercise2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let (Some(s), n) = (six,none) {
        println!("{:?}, {:?}", s, n);

        println!("Success!");
        return
    }

    panic!("NEVER LET THIS RUN！");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}


use List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }
    fn append(self, elem:u32) -> List {
        match self {
            Cons(head, tail) => Cons(head, Box::new(tail.append(elem))),
            Nil => Cons(elem, Box::new(Nil)),
        }
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // After Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn linked_list() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(3);
    list = list.prepend(2);
    list = list.prepend(1);
    list = list.append(4);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
