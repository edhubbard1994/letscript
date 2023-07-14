use regex::Regex;

use crate::token::{Token, TokenType, TokenValue};

#[derive(Clone, Copy, Debug)]
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
    SubtractF64I32(f64, i32),
    MultiplyI32(i32, i32),
    MultiplyI32F64(i32, f64),
    MultiplyF64(f64, f64),
    DivideI32(i32, i32),
    DivideI32F64(i32, f64),
    DivideF64I32(f64, i32),
    DivideF64(f64, f64),
    ModI32(i32, i32),
    ModI32F64(i32, f64),
    ModF64I32(f64, i32),
    ModF64(f64, f64),
    And(bool, bool),
    Or(bool, bool),
    Equal(bool, bool),
    NotEqual(bool, bool),
    Gt(bool, bool),
    Lt(bool, bool),
    Gte(bool, bool),
    Lte(bool, bool),
    EqualI32(i32, i32),
    NotEqualI32(i32, i32),
    GtI32(i32, i32),
    LtI32(i32, i32),
    GteI32(i32, i32),
    LteI32(i32, i32),
    EqualF64(f64, f64),
    NotEqualF64(f64, f64),
    GtF64(f64, f64),
    LtF64(f64, f64),
    GteF64(f64, f64),
    LteF64(f64, f64),
    Negate(bool),
}

impl LSExpr {
    fn eval(&self) -> LSExpr {
        match self {
            LSExpr::AddF64(l, r) => return LSExpr::Float64(l + r),
            LSExpr::AddI32F64(l, r) => return LSExpr::Float64(*l as f64 + r),
            LSExpr::AddI32(l, r) => return LSExpr::Int32(l + r),
            LSExpr::SubtractF64(l, r) => return LSExpr::Float64(l - r),
            LSExpr::SubtractI32F64(l, r) => return LSExpr::Float64(*l as f64 - r),
            LSExpr::SubtractF64I32(l, r) => return LSExpr::Float64(*l - *r as f64),
            LSExpr::SubtractI32(l, r) => return LSExpr::Int32(l - r),
            LSExpr::MultiplyF64(l, r) => return LSExpr::Float64(l * r),
            LSExpr::MultiplyI32F64(l, r) => return LSExpr::Float64(*l as f64 * r),
            LSExpr::MultiplyI32(l, r) => return LSExpr::Int32(l * r),
            LSExpr::DivideF64(l, r) => return LSExpr::Float64(l / r),
            LSExpr::DivideI32F64(l, r) => return LSExpr::Float64(*l as f64 / r),
            LSExpr::DivideI32(l, r) => return LSExpr::Int32(l / r),
            LSExpr::DivideF64I32(l, r) => return LSExpr::Float64(*l / *r as f64),
            LSExpr::ModI32(l, r) => return LSExpr::Int32(*l % *r),
            LSExpr::ModF64(l, r) => return LSExpr::Float64(*l % *r),
            LSExpr::ModI32F64(l, r) => return LSExpr::Float64(*l as f64 % *r),
            LSExpr::ModF64I32(l, r) => return LSExpr::Float64(*l % *r as f64),
            LSExpr::And(l, r) => return LSExpr::Boolean(*l && *r),
            LSExpr::Or(l, r) => return LSExpr::Boolean(*l || *r),
            LSExpr::Equal(l, r) => return LSExpr::Boolean(*l == *r),
            LSExpr::NotEqual(l, r) => return LSExpr::Boolean(*l != *r),
            LSExpr::Gt(l, r) => return LSExpr::Boolean(*l > *r),
            LSExpr::Lt(l, r) => return LSExpr::Boolean(*l < *r),
            LSExpr::Gte(l, r) => return LSExpr::Boolean(*l >= *r),
            LSExpr::Lte(l, r) => return LSExpr::Boolean(*l <= *r),
            LSExpr::EqualI32(l, r) => return LSExpr::Boolean(*l == *r),
            LSExpr::NotEqualI32(l, r) => return LSExpr::Boolean(*l != *r),
            LSExpr::GtI32(l, r) => return LSExpr::Boolean(*l > *r),
            LSExpr::LtI32(l, r) => return LSExpr::Boolean(*l < *r),
            LSExpr::GteI32(l, r) => return LSExpr::Boolean(*l >= *r),
            LSExpr::LteI32(l, r) => return LSExpr::Boolean(*l <= *r),
            LSExpr::EqualF64(l, r) => return LSExpr::Boolean(*l == *r),
            LSExpr::NotEqualF64(l, r) => return LSExpr::Boolean(*l != *r),
            LSExpr::GtF64(l, r) => return LSExpr::Boolean(*l > *r),
            LSExpr::LtF64(l, r) => return LSExpr::Boolean(*l < *r),
            LSExpr::GteF64(l, r) => return LSExpr::Boolean(*l >= *r),
            LSExpr::LteF64(l, r) => return LSExpr::Boolean(*l <= *r),
            LSExpr::Negate(v) => return LSExpr::Boolean(!*v),
            _ => return LSExpr::Undefined,
        };
    }
    fn cast_to_bool(&self) -> LSExpr {
        match self {
            LSExpr::Int32(x) => {
                if *x == 0 {
                    LSExpr::Boolean(false)
                } else {
                    LSExpr::Boolean(true)
                }
            }
            LSExpr::Float64(x) => {
                if *x == 0.0 {
                    LSExpr::Boolean(false)
                } else {
                    LSExpr::Boolean(true)
                }
            }
            LSExpr::Null => LSExpr::Boolean(false),
            LSExpr::Boolean(x) => LSExpr::Boolean(*x),
            _ => LSExpr::Undefined,
        }
    }

    fn cast_to_float(&self) -> LSExpr {
        match self {
            LSExpr::Int32(x) => LSExpr::Float64(*x as f64),
            LSExpr::Float64(x) => LSExpr::Float64(*x),
            LSExpr::Null => LSExpr::Null,
            LSExpr::Boolean(x) => {
                if *x == false {
                    LSExpr::Float64(1.0)
                } else {
                    LSExpr::Float64(0.0)
                }
            }
            _ => LSExpr::Undefined,
        }
    }

    fn cast_to_int(&self) -> LSExpr {
        match self {
            LSExpr::Int32(x) => LSExpr::Int32(*x),
            LSExpr::Float64(x) => LSExpr::Int32(*x as i32),
            LSExpr::Null => LSExpr::Null,
            LSExpr::Boolean(x) => {
                if *x == false {
                    LSExpr::Int32(1)
                } else {
                    LSExpr::Int32(0)
                }
            }
            _ => LSExpr::Undefined,
        }
    }
}

fn convert_token_to_value(t: Token) -> LSExpr {
    let int_regx = Regex::new(r"^(\d+\b|-\d+\b)$").unwrap();
    let float_regx = Regex::new(r"^(\d+\.\d+\b|-\d+\.\d+\b)$").unwrap();
    let bool_regx = Regex::new(r"^(true\b|false\b)$").unwrap();
    let null_regx = Regex::new(r"^null$").unwrap();
    let undef_regx = Regex::new(r"^undefined$").unwrap();
    let val = t.tok_value.unwrap().s_val.unwrap();
    println!("{}", val);
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
    let mut expr = LSExpr::Undefined;
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
        (LSExpr::Int32(x), LSExpr::Int32(y), TokenType::Minus) => {
            expr = LSExpr::SubtractI32(x, y).eval()
        }
        (LSExpr::Int32(x), LSExpr::Float64(y), TokenType::Minus) => {
            expr = LSExpr::SubtractI32F64(x, y).eval()
        }
        (LSExpr::Float64(x), LSExpr::Int32(y), TokenType::Minus) => {
            expr = LSExpr::SubtractI32F64(y, x).eval()
        }
        (LSExpr::Float64(x), LSExpr::Float64(y), TokenType::Minus) => {
            expr = LSExpr::SubtractF64(y, x).eval()
        }
        (LSExpr::Int32(x), LSExpr::Int32(y), TokenType::Mult) => {
            expr = LSExpr::MultiplyI32(x, y).eval()
        }
        (LSExpr::Int32(x), LSExpr::Float64(y), TokenType::Mult) => {
            println!("x:{}\ny:{}", x, y);
            expr = LSExpr::MultiplyI32F64(x, y).eval();
            println!("expr:{:?}", &expr);
        }
        (LSExpr::Float64(x), LSExpr::Int32(y), TokenType::Mult) => {
            expr = LSExpr::MultiplyI32F64(y, x).eval()
        }
        (LSExpr::Float64(x), LSExpr::Float64(y), TokenType::Mult) => {
            expr = LSExpr::MultiplyF64(y, x).eval()
        }
        (LSExpr::Int32(x), LSExpr::Int32(y), TokenType::Div) => {
            expr = LSExpr::DivideI32(x, y).eval()
        }
        (LSExpr::Int32(x), LSExpr::Float64(y), TokenType::Div) => {
            expr = LSExpr::DivideI32F64(x, y).eval()
        }
        (LSExpr::Float64(x), LSExpr::Int32(y), TokenType::Div) => {
            expr = LSExpr::DivideF64I32(x, y).eval()
        }
        (LSExpr::Float64(x), LSExpr::Float64(y), TokenType::Div) => {
            expr = LSExpr::DivideF64(y, x).eval()
        }
        (LSExpr::Int32(x), LSExpr::Int32(y), TokenType::Mod) => expr = LSExpr::ModI32(x, y).eval(),
        (LSExpr::Int32(x), LSExpr::Float64(y), TokenType::Mod) => {
            expr = LSExpr::ModI32F64(x, y).eval()
        }
        (LSExpr::Float64(x), LSExpr::Int32(y), TokenType::Mod) => {
            expr = LSExpr::ModI32F64(y, x).eval()
        }
        (LSExpr::Float64(x), LSExpr::Float64(y), TokenType::Mod) => {
            expr = LSExpr::ModF64(y, x).eval()
        }

        (LSExpr::Boolean(x), LSExpr::Boolean(y), TokenType::And) => expr = LSExpr::And(y, x).eval(),
        (LSExpr::Boolean(x), LSExpr::Boolean(y), TokenType::Or) => expr = LSExpr::Or(y, x).eval(),
        (_, _, TokenType::And) => {
            expr = LSExpr::And(
                match l.cast_to_bool() {
                    LSExpr::Boolean(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_bool() {
                    LSExpr::Boolean(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::Or) => {
            expr = LSExpr::Or(
                match l.cast_to_bool() {
                    LSExpr::Boolean(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_bool() {
                    LSExpr::Boolean(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::GreaterThan) => {
            expr = LSExpr::GtF64(
                match l.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::LessThan) => {
            expr = LSExpr::LtF64(
                match l.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::Gte) => {
            expr = LSExpr::GteF64(
                match l.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::Lte) => {
            expr = LSExpr::LteF64(
                match l.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        (_, _, TokenType::Equals) => {
            expr = LSExpr::EqualF64(
                match l.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
                match r.cast_to_float() {
                    LSExpr::Float64(x) => x,
                    _ => panic!("got non bool value"),
                },
            )
            .eval()
        }
        _ => {
            println!(
                "l: {:?}, r: {:?}, op: {:?} is not a valid binary expr",
                l, r, operator.tok_type
            );
            expr = LSExpr::Undefined;
        }
    }
    return Token {
        tok_type: TokenType::Literal,
        tok_value: Some(TokenValue {
            s_val: match expr {
                LSExpr::Int32(x) => Some(x.to_string()),
                LSExpr::Float64(x) => Some(x.to_string()),
                LSExpr::Boolean(x) => Some(x.to_string()),
                LSExpr::Null => None,
                _ => Some(String::from("Undefined")),
            },
        }),
    };
}

pub fn operate_unary(value: Token, operator: Token) -> Token {
    let val = convert_token_to_value(value);
    let expr;
    match operator.tok_type {
        TokenType::Not => {
            let b_val = val.cast_to_bool();
            expr = match b_val {
                LSExpr::Boolean(v) => LSExpr::Negate(v).eval(),
                _ => {
                    panic!("unknown unary type")
                }
            };
        }
        _ => {
            panic!("unknown unary")
        }
    }

    return Token {
        tok_type: TokenType::Literal,
        tok_value: Some(TokenValue {
            s_val: match expr {
                LSExpr::Int32(x) => Some(x.to_string()),
                LSExpr::Float64(x) => Some(x.to_string()),
                LSExpr::Boolean(x) => Some(x.to_string()),
                LSExpr::Null => None,
                _ => Some(String::from("Undefined")),
            },
        }),
    };
}
