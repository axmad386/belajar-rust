// associated function semacam static method kalau di oop
// ditandai dengan impl
pub fn run() {
    basic();
    example();
    multiple_impl();
}

fn basic() {
    struct Point {
        x: f64,
        y: f64,
    }

    // Implementation block, all `Point` associated functions & methods go in here.
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // Another associated function, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }
    let p1 = Point::origin();
    let p2 = Point::new(3.0, 4.0);
    println!("p1.x = {}, p2.y = {}", p1.x, p2.y);
}

fn example() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        // 1. Implement an associated function `new`,
        // 2. It will return a TrafficLight contains color "red"
        // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
        pub fn new() -> Self {
            Self {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

fn multiple_impl() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Using multiple `impl` blocks to rewrite the code below.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let r = Rectangle {
        width: 10,
        height: 20,
    };
    let smaller = Rectangle {
        width: 5,
        height: 10,
    };
    let area = r.area();
    println!("area: {}", area);
    assert!(r.can_hold(&smaller));
    println!("Success!");
}

