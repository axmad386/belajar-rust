use clap::Parser;
mod basic_types;
mod compound;
mod conversion;
mod enums;
mod expressions;
mod hashmap;
mod ownership;
mod ref_borrow;
mod string;
mod traits;
mod tuples;
mod variables;
mod vecs;
mod flow;
mod pattern;
mod function;
mod generic;

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
        "variables" => match args.submodule.as_deref() {
            Some("mutability") => variables::mutability::run(),
            Some("scope") => variables::scope::run(),
            Some("shadowing") => variables::shadowing::run(),
            Some("destructuring") => variables::destructuring::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "basic_types" => match args.submodule.as_deref() {
            Some("number") => basic_types::number::run(),
            Some("char") => basic_types::char::run(),
            Some("bool") => basic_types::bool::run(),
            Some("unit") => basic_types::unit::run(),
            Some("statement_expression") => basic_types::statement_expression::run(),
            Some("function") => basic_types::function::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "ownership" => ownership::run(),
        "ref_borrow" => ref_borrow::run(),
        "compound" => match args.submodule.as_deref() {
            Some("string") => compound::string::run(),
            Some("array") => compound::array::run(),
            Some("slice") => compound::slice::run(),
            Some("tuple") => compound::tuple::run(),
            Some("structs") => compound::structs::run(),
            Some("enums") => compound::enums::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "flow" => match args.submodule.as_deref() {
            Some("if_else") => flow::if_else::run(),
            Some("for_in") => flow::for_in::run(),
            Some("while_loop") => flow::while_loop::run(),
            Some("loops") => flow::loops::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "pattern" => match args.submodule.as_deref() {
            Some("match_expression") => pattern::match_expression::run(),
            Some("patterns") => pattern::patterns::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "function" => match args.submodule.as_deref(){
            Some("associated") => function::associated::run(),
            Some("method") => function::method::run(),
            _ => unreachable!("submodule tidak ditemukan"),
        },
        "generic" => match args.submodule.as_deref(){
            Some("functions") => generic::functions::run(),
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
