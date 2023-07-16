use crate::token::Token;
use crate::token::TokenType;
use lazy_static::lazy_static;
use phf::Map;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::ops::DerefMut;
use std::sync::Mutex;

lazy_static! {


    #[derive(Clone, Debug)]
    pub static ref CALL_STACK: Mutex<LinkedList<Scope>> = Mutex::new(LinkedList::<Scope>::new());
    static ref FUNCTION_TABLE: Mutex<Map<String, Vec<Token>>> = Mutex::new(Map::<String, Vec<Token>>::new());
}

#[derive(Clone, Debug)]
pub enum SymbolType {
    Number(String),
    String(String),
    Object(ObjectSymbolArgs),
    Array(ArraySymbolArgs),
    Function(FunctionSymbolArgs),
    Pointer(String),
}

#[derive(Clone, Debug)]
pub struct FunctionSymbolArgs {
    pub args: Vec<Token>,
    pub body: LinkedList<Token>,
}

#[derive(Clone, Debug)]
pub struct ObjectSymbolArgs {
    pub vars: HashMap<String, SymbolType>,
    pub methods: HashMap<String, FunctionSymbolArgs>,
}

#[derive(Clone, Debug)]
pub struct ArraySymbolArgs {
    pub array: Vec<SymbolType>,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub symbols: HashMap<String, SymbolType>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, symbol: SymbolType) {
        self.symbols.insert(name, symbol);
    }
}

impl CALL_STACK {
    pub fn push(&self, scope: Scope) {
        CALL_STACK.lock().unwrap().push_back(scope);
    }

    pub fn pop(&self) {
        CALL_STACK.lock().unwrap().pop_back();
    }

    pub fn add_symbol(&self, name: String, symbol: SymbolType) {
        let mut stack = CALL_STACK.lock().unwrap();
        if stack.is_empty() {
            stack.push_back(Scope::new());
        }
        let mut scope = stack.back_mut().unwrap();
        scope.add(name, symbol);
        println!("{:?}", scope);
    }

    pub fn lookup_symbol(&self, name: String) -> Option<SymbolType> {
        let stack = CALL_STACK.lock().unwrap();
        for scope in stack.iter().rev() {
            if let Some(symbol) = scope.symbols.get(&name) {
                return Some(symbol.clone());
            }
        }
        None
    }
}
