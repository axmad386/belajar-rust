fn main() {
    let first_letter = "Halo Indonesia";
    let second_letter = "hallo";
    let longest = longest(first_letter, second_letter);

    match longest {
        StrOrInt::Int(value) => println!("integer {}", value),
        StrOrInt::Str(value) => println!("string {}", value),
    }
}

enum StrOrInt<'a> {
    Str(&'a str),
    Int(i32),
}

fn longest<'a>(first: &'a str, second: &'a str) -> StrOrInt<'a> {
    if first.len() > second.len() {
        StrOrInt::Str(first)
    } else {
        StrOrInt::Int(0)
    }
}
