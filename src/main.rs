use clap::Parser;
mod conversion;
mod enums;
mod expressions;
mod hashmap;
mod string;
mod traits;
mod tuples;
mod variables;
mod vecs;
mod basic_types;

fn main() {
    let args = Cli::parse();
    println!("{}", args.submodule.as_deref().unwrap());
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
        "variables" => match args.submodule.as_deref() {
            Some("mutability") => variables::mutability::run(),
            Some("scope") => variables::scope::run(),
            Some("shadowing") => variables::shadowing::run(),
            Some("destructuring") => variables::destructuring::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "basic_types" => match args.submodule.as_deref(){
            Some("number") => basic_types::number::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        }
        _ => unreachable!("module tidak ditemukan"),
    }
}

#[derive(Parser)]
#[clap(author = "Akhmad Salafudin", about = "Belajar rust dengan CLI")]
struct Cli {
    module: String,
    submodule: Option<String>,
}
