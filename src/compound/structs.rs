pub fn run() {
    basic();
    struct_trait();
    tupple_struct();
    mutable_struct();
    field_init();
    update_struct();
    print_struct();
    partial_move();
    excercise();
}

// Fix the error
fn basic() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "music".to_string(),
    };
    println!("{:?}", p);
    let Person { name, age, hobby } = p;
    println!("{name}, {age}, {hobby}");

    println!("Success!");
}

struct Unit;
trait Hello {
    fn say_hello(&self) -> ();
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl Hello for Unit {
    fn say_hello(&self) -> () {
        println!("hello from unit")
    }
}
fn struct_trait() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: Unit) {
    // run say_hello
    u.say_hello();
}

// Fix the error and fill the blanks
struct Color(i32, i32, i32);
fn tupple_struct() {
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, _y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

fn mutable_struct() {
    // Fill the blank and fix the error without adding/removing new line
    struct Person {
        name: String,
        age: u8,
    }
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success!");
}
fn field_init() {
    // Fill the blank
    struct Person {
        name: String,
        age: u8,
    }

    let p = build_person("Foo".to_string(), 20);
    println!("{} is {}", p.name, p.age);
    fn build_person(name: String, age: u8) -> Person {
        Person { age, name }
    }
}

// Fill the blank to make the code work
fn update_struct() {
    #[derive(Debug, PartialEq)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("{:?}", u2);
    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

// Fill the blanks to make the code work
fn print_struct() {
    #[derive(Debug, PartialEq)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // Print debug info to stderr

    println!("{:?}", rect1); // Print debug info to stdout
}

fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

// Fix errors to make it work
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn excercise() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    // ONLY modify this line
    println!("{}, {}",_name, f.data);
} 
