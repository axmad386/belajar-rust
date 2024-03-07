pub fn run() {
    basic();
    expression();
    enums();
    matches();
    matches_enum();
    if_let();
    if_let_excercise();
    match_excercise();
    shadowing();
}

#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn basic() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        }
        _ => println!("West"),
    };
}

fn expression() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}
// Fill in the blanks
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enums() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
        Message::Write("hallo".to_string()),
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            Message::Write(s) => {
                assert_eq!(s, "hallo");
            }
            __ => println!("no data in these variants"),
        }
    }
}
fn matches() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a' | 'E' | 'Z' | '0' | 'x' | '9' | 'Y'));
    }

    println!("Success!");
}

fn matches_enum() {
    enum MyEnum {
        Foo,
        Bar,
    }

    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success!");
}

fn if_let() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    // match o {
    //     Some(i) => {
    //         println!("This is a really long string and `{:?}`", i);
    //
    //         println!("Success!");
    //     }
    //     _ => {}
    // };
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}

// Fill in the blank
fn if_let_excercise() {
    enum Foo {
        Bar(u8),
    }
    let a = Foo::Bar(1);

    #[allow(irrefutable_let_patterns)]
    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

fn match_excercise() {
    #[allow(dead_code)]
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead
    // if let Foo::Bar = a {
    //     println!("match foo::bar")
    // } else if let Foo::Baz = a {
    //     println!("match foo::baz")
    // } else {
    //     println!("match others")
    // }
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

// Fix the errors in-place
fn shadowing() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
       assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here
    
    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }
