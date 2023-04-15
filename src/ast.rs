use phf::Map;
use std::collections::HashMap;
use std::collections::LinkedList;
use crate::token::Token;
use crate::token::TokenType;


static mut CALL_STACK: LinkedList<&Scope> = LinkedList::<&Scope>::new();
static mut  FUNCTION_TABLE: Map<String,LinkedList<Token>> = Map::<String,LinkedList<Token>>::new();

pub enum Symbol {
    Variable(String),
    Function(String),

}

pub struct Scope {
    symbols: HashMap<String,Symbol>
}


impl Scope {
    pub fn add_var(&mut self ,name: Token, value: Token) {
        let (n,v) = match (name.tok_type, value.tok_type ){
           ( TokenType::Literal, TokenType::Literal )=> (name.tok_value.unwrap().s_val.unwrap(),value.tok_value.unwrap().s_val.unwrap()),
            (_,_) => panic!("keyword cannot be used as a variable name")
        };

        if self.symbols.contains_key(&n) {
            panic!("variable name {} is already in use",n);
        }

        self.symbols.insert(n, Symbol::Variable(v));
    }
}

