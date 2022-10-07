use regex::Regex;
use sha2::digest::generic_array::typenum::Or;

use crate::token::{Token, TokenType, TokenValue};

pub enum LSExpr {
    Int32(i32),
    Float64(f64),
    Boolean(bool),
    Undefined,
    Null,
    AddI32(i32, i32),
    AddI32F64(i32, f64),
    AddF64(f64, f64),
    SubtractI32(i32, i32),
    SubtractF64(f64, f64),
    SubtractI32F64(i32, f64),
    MultiplyI32(i32, i32),
    MultiplyI32F64(i32, f64),
    MultiplyF64(f64, f64),
    DivideI32(i32, i32),
    DivideI32F64(i32, f64),
    DivideF64(f64, f64),
    ModI32(i32, i32),
    ModI32F64(i32, f64),
    ModF64(f64, f64),
    And(bool, bool),
    Or(bool, bool),
}

impl LSExpr {
    fn eval(&self) -> LSExpr {
        match self {
            LSExpr::AddF64(l, r) => return LSExpr::Float64(l + r),
            LSExpr::AddI32F64(l, r) => return LSExpr::Float64(*l as f64 + r),
            LSExpr::AddI32(l, r) => return LSExpr::Int32(l + r),
            LSExpr::SubtractF64(l, r) => return LSExpr::Float64(l - r),
            LSExpr::SubtractI32F64(l, r) => return LSExpr::Float64(*l as f64 - r),
            LSExpr::SubtractI32(l, r) => return LSExpr::Int32(l - r),

            _ => return LSExpr::Undefined,
        };
    }
}

fn convert_token_to_value(t: Token) -> LSExpr {
    let int_regx = Regex::new(r"\d+\z").unwrap();
    let float_regx = Regex::new(r"\d+\.\d+\z").unwrap();
    let bool_regx = Regex::new(r"(true\b|false\b)").unwrap();
    let null_regx = Regex::new(r"null\z").unwrap();
    let undef_regx = Regex::new(r"undefined\z").unwrap();
    let val = t.tok_value.unwrap().s_val.unwrap();
    if int_regx.is_match(&val) {
        return LSExpr::Int32(val.parse::<i32>().ok().unwrap());
    } else if float_regx.is_match(&val) {
        return LSExpr::Float64(val.parse::<f64>().ok().unwrap());
    } else if bool_regx.is_match(&val) {
        return LSExpr::Boolean(val.parse::<bool>().ok().unwrap());
    } else if null_regx.is_match(&val) {
        return LSExpr::Null;
    } else {
        return LSExpr::Undefined;
    }
}

pub fn operate(left: Token, right: Token, operator: Token) -> Token {
    let l = convert_token_to_value(left);
    let r = convert_token_to_value(right);
    let mut expr: LSExpr;
    match (l, r, operator.tok_type) {
        (LSExpr::Int32(x), LSExpr::Int32(y), TokenType::Plus) => expr = LSExpr::AddI32(x, y).eval(),
        (LSExpr::Int32(x), LSExpr::Float64(y), TokenType::Plus) => {
            expr = LSExpr::AddI32F64(x, y).eval()
        }
        (LSExpr::Float64(x), LSExpr::Int32(y), TokenType::Plus) => {
            expr = LSExpr::AddI32F64(y, x).eval()
        }
        (LSExpr::Float64(x), LSExpr::Float64(y), TokenType::Plus) => {
            expr = LSExpr::AddF64(y, x).eval()
        }
        _ => {
            todo!("return error");
        }
    }
    return Token {
        tok_type: TokenType::Assign,
        tok_value: None,
    };
}
