use crate::{
    ast::{Scope, SymbolType},
    tokenizer::tokenize,
};

#[test]
pub fn test_scope_add_number() {
    let mut scope = Scope::new();
    let mut input = String::from("var x is 3");
    let tokens = tokenize(&mut input);
    scope.add_number(tokens[1].clone(), tokens[3].clone());
    let val = scope.symbols.get("x").unwrap();
    match val {
        SymbolType::Number(n) => assert_eq!(n, "3"),
        _ => panic!("expected number"),
    }
}
