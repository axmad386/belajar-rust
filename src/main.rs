mod enums;
mod expressions;
mod hashmap;
mod string;
mod tuples;
mod vecs;

fn main() {
    enums::run();
    vecs::run();
    string::run();
    hashmap::run();
    tuples::run();
    expressions::run();
}
