use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
 type Output = FooBar;

 fn add(self, _rhs: Bar) -> Self::Output {
     println!("> Foo.add(Bar) was called");

     FooBar
 }
}


impl ops::Add<Foo> for Bar {
 type Output = BarFoo;

 fn add(self, _rhs: Foo) -> Self::Output {
     println!("> Bar.add(Foo) was called");

     BarFoo
 }
}

pub fn run() {
    println!("====Operator Overleading===");

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}",  Bar + Foo);
}
