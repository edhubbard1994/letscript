use crate::token::Token;
use crate::token::TokenType;
use phf::Map;
use std::collections::HashMap;
use std::collections::LinkedList;

static mut CALL_STACK: LinkedList<&Scope> = LinkedList::<&Scope>::new();
static mut FUNCTION_TABLE: Map<String, LinkedList<Token>> = Map::<String, LinkedList<Token>>::new();

pub enum SymbolType {
    Number(String),
    String(String),
    Object(ObjectSymbolArgs),
    Array(ArraySymbolArgs),
    Function(FunctionSymbolArgs),
}

pub struct FunctionSymbolArgs {
    pub args: Vec<Token>,
    pub body: LinkedList<Token>,
}

pub struct ObjectSymbolArgs {
    pub vars: HashMap<String, SymbolType>,
    pub methods: HashMap<String, FunctionSymbolArgs>,
}

pub struct ArraySymbolArgs {
    pub array: Vec<SymbolType>,
}

pub struct Scope {
    symbols: HashMap<String, SymbolType>,
}

impl Scope {
    pub fn add_number(&mut self, name: Token, value: Token) {
        let (n, v) = match (name.tok_type, value.tok_type) {
            (TokenType::Literal, TokenType::Literal) => (
                name.tok_value.unwrap().s_val.unwrap(),
                value.tok_value.unwrap().s_val.unwrap(),
            ),
            (_, _) => panic!("keyword cannot be used as a variable name"),
        };

        if self.symbols.contains_key(&n) {
            panic!("variable name {} is already in use", n);
        }

        self.symbols.insert(n, SymbolType::Number(v));
    }

    pub fn add_string(&mut self, name: Token, value: Token) {
        let (n, v) = match (name.tok_type, value.tok_type) {
            (TokenType::Literal, TokenType::Literal) => (
                name.tok_value.unwrap().s_val.unwrap(),
                value.tok_value.unwrap().s_val.unwrap(),
            ),
            (_, _) => panic!("keyword cannot be used as a variable name"),
        };

        if self.symbols.contains_key(&n) {
            panic!("variable name {} is already in use", n);
        }

        self.symbols.insert(n, SymbolType::String(v));
    }

    pub fn add_object(&mut self, name: Token, value_pairs: Vec<(Token, Token)>) {
        let n = match name.tok_type {
            TokenType::Literal => name.tok_value.unwrap().s_val.unwrap(),
            _ => panic!("keyword cannot be used as a variable name"),
        };

        if self.symbols.contains_key(&n) {
            panic!("variable name {} is already in use", n);
        }

        let mut vars: HashMap<String, SymbolType> = HashMap::new();
        let mut methods: HashMap<String, FunctionSymbolArgs> = HashMap::new();

        for (k, v) in value_pairs {
            match k.tok_type {
                TokenType::Literal => {
                    let key = k.tok_value.unwrap().s_val.unwrap();
                    match v.tok_type {
                        TokenType::Literal => {
                            let val = v.tok_value.unwrap().s_val.unwrap();
                            vars.insert(key, SymbolType::String(val));
                        }
                        TokenType::Number => {
                            let val = v.tok_value.unwrap().n_val.unwrap();
                            vars.insert(key, SymbolType::Number(val));
                        }
                        TokenType::Object => {
                            let val = v.tok_value.unwrap().o_val.unwrap();
                            vars.insert(key, SymbolType::Object(val));
                        }
                        TokenType::Array => {
                            let val = v.tok_value.unwrap().a_val.unwrap();
                            vars.insert(key, SymbolType::Array(val));
                        }
                        TokenType::Function => {
                            let val = v.tok_value.unwrap().f_val.unwrap();
                            methods.insert(key, val);
                        }
                        _ => panic!("invalid value type for object"),
                    }
                }
                _ => panic!("invalid key type for object"),
            }
        }

        self.symbols
            .insert(n, SymbolType::Object(ObjectSymbolArgs { vars, methods }));
    }

    pub fn add_array(name: Token, values: Vec<Token>) {
        let n = match name.tok_type {
            TokenType::Literal => name.tok_value.unwrap().s_val.unwrap(),
            _ => panic!("keyword cannot be used as a variable name"),
        };

        if self.symbols.contains_key(&n) {
            panic!("variable name {} is already in use", n);
        }

        let mut array: Vec<SymbolType> = Vec::new();

        for v in values {
            match v.tok_type {
                TokenType::Literal => {
                    let val = v.tok_value.unwrap().s_val.unwrap();
                    array.push(SymbolType::String(val));
                }
                TokenType::Number => {
                    let val = v.tok_value.unwrap().n_val.unwrap();
                    array.push(SymbolType::Number(val));
                }
                TokenType::Object => {
                    let val = v.tok_value.unwrap().o_val.unwrap();
                    array.push(SymbolType::Object(val));
                }
                TokenType::Array => {
                    let val = v.tok_value.unwrap().a_val.unwrap();
                    array.push(SymbolType::Array(val));
                }
                TokenType::Function => {
                    let val = v.tok_value.unwrap().f_val.unwrap();
                    array.push(SymbolType::Function(val));
                }
                _ => panic!("invalid value type for array"),
            }
        }

        self.symbols
            .insert(n, SymbolType::Array(ArraySymbolArgs { array }));
    }
}
