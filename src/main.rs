mod ast;
mod expr;
mod parser;
mod test_ast;
mod test_expr;
mod test_parser;
mod token;
mod tokenizer;

use std::env;

// use uwl::StringStream;

fn parse_args(args: Vec<String>) {}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{}", input);
        let mut tokens = tokenizer::tokenize(&mut input);
        let pasrsed_tokens = parser::parse(&mut tokens);
        println!("{:?}", tokens.len());
        //let ast = parser::parse(&mut tokens);
        //println!("{:?}", ast);
    }
}
