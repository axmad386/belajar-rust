use clap::Parser;
mod conversion;
mod enums;
mod expressions;
mod hashmap;
mod string;
mod traits;
mod tuples;
mod vecs;

fn main() {
    let args = Cli::parse();
    match args.module.as_str() {
        "conversion" => conversion::run(),
        "enums" => enums::run(),
        "expressions" => expressions::run(),
        "hashmap" => hashmap::run(),
        "string" => string::run(),
        "traits" => match args.submodule.as_deref() {
            Some("basic") => traits::basic::run(),
            Some("derive") => traits::derive::run(),
            Some("dynamic") => traits::dynamic::run(),
            Some("overload") => traits::overload::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "tuples" => tuples::run(),
        "vecs" => vecs::run(),
        _ => unreachable!("module tidak ditemukan"),
    }
}

#[derive(Parser)]
#[clap(author = "Akhmad Salafudin", about = "Belajar rust dengan CLI")]
struct Cli {
    module: String,
    submodule: Option<String>,
}
