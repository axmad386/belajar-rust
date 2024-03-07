pub fn run() {
    basic();
    enums();
    borrow();
    selfs();
}

fn basic() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Complete the area method which return the area of a Rectangle.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    assert_eq!(rect1.area(), 1500);

    println!("Success!");
}
fn enums() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // Implement TrafficLightColor with a method.
    impl TrafficLightColor {
        fn color(&self) -> &str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
fn borrow() {
    // Only fill in the blanks, DON'T remove any line!
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}
fn selfs() {
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        // Using `Self` to fill in the blank.
        pub fn show_state(self: &Self) {
            println!("the current state is {}", self.color);
        }

        // Fill in the blank, DON'T use any variants of `Self`.
        pub fn change_state(&mut self) {
            self.color = match self.color.as_str() {
                "red" =>"green".to_string(),
                "green" => "yellow".to_string(),
                "yellow" => "red".to_string(),
                _ => "red".to_string(),
            }
        }
    }

    let mut light = TrafficLight {
        color: "red".to_owned(),
    };
    light.show_state();
    light.change_state();
    light.show_state();
    println!("Success!");
}
