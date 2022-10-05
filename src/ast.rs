use std::any::Any;

use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenValue;
use regex::Regex;
use sha2::Digest;
use sha2::Sha256;

pub trait LSObject {
    fn hash(&self) -> String;
    fn to_string(&self) -> String;
    fn get_type(&self) -> String;
    fn get_name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait LSValue {
    fn get_value(&self) -> String;
    fn get_value_type(&self) -> String;
    fn cast_to_int(&self) -> IntValue;
    fn cast_to_float(&self) -> FloatValue;
    fn cast_to_bool(&self) -> BoolValue;
}

pub unsafe trait Operable {
    fn add(&self, right: &impl LSValue) -> Box<&dyn LSValue>;
    fn subtract(&self, right: &impl LSValue) -> Box<&Self>;
    fn multiply(&self, right: &impl LSValue) -> Box<&Self>;
    fn divide(&self, right: &impl LSValue) -> Box<&Self>;
    fn modulus(&self, right: &impl LSValue) -> Box<&Self>;
    fn and(&self, right: &impl LSValue) -> Box<&Self>;
    fn or(&self, right: &impl LSValue) -> Box<&Self>;
}

struct IntValue {
    val: i64,
}

impl LSObject for IntValue {
    fn hash(&self) -> String {
        let sha = Sha256::new();
        sha.update(self.val.to_string());
        return format!("{:X}", sha.finalize());
    }
    fn to_string(&self) -> String {
        return String::from(self.val.to_string());
    }
    fn get_type(&self) -> String {
        return String::from("Integer");
    }
    fn get_name(&self) -> String {
        return format!("IntegerValue({})", &self.val);
    }

    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl LSValue for IntValue {
    fn cast_to_int(&self) -> IntValue {
        return *self.clone();
    }

    fn cast_to_float(&self) -> FloatValue {
        return FloatValue {
            val: self.val as f64,
        };
    }

    fn cast_to_bool(&self) -> BoolValue {
        let mut bool_val;
        match self.val {
            0 => bool_val = false,
            _ => bool_val = true,
        }
        return BoolValue { val: bool_val };
    }

    fn get_value_type(&self) -> String {
        return self.get_type();
    }

    fn get_value(&self) -> String {
        return self.val.to_string();
    }
}

unsafe impl Operable for IntValue {
    fn add(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" {
            return Box::new(&IntValue {
                val: self.val + right.cast_to_int().val,
            });
        } else if right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: (self.val as f64) + right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn subtract(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" {
            return Box::new(&IntValue {
                val: self.val - right.cast_to_int().val,
            });
        } else if right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: (self.val as f64) - right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn multiply(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" {
            return Box::new(&IntValue {
                val: self.val * right.cast_to_int().val,
            });
        } else if right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: (self.val as f64) * right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn divide(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" {
            return Box::new(&IntValue {
                val: self.val / right.cast_to_int().val,
            });
        } else if right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: (self.val as f64) / right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn modulus(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" {
            return Box::new(&IntValue {
                val: self.val % right.cast_to_int().val,
            });
        } else if right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: (self.val as f64) % right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn and(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }

    fn or(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }
}

struct FloatValue {
    val: f64,
}

impl LSObject for FloatValue {
    fn hash(&self) -> String {
        let sha = Sha256::new();
        sha.update(self.val.to_string());
        return format!("{:X}", sha.finalize());
    }

    fn to_string(&self) -> String {
        return self.val.to_string();
    }

    fn get_type(&self) -> String {
        return String::from("Float");
    }

    fn get_name(&self) -> String {
        return format!("FloatValue({})", &self.val);
    }
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl LSValue for FloatValue {
    fn get_value_type(&self) -> String {
        return self.get_type();
    }

    fn cast_to_int(&self) -> IntValue {
        return IntValue {
            val: self.val as i64,
        };
    }

    fn cast_to_float(&self) -> FloatValue {
        return *self.clone();
    }

    fn cast_to_bool(&self) -> BoolValue {
        if self.val == 0.0 {
            return BoolValue { val: false };
        }
        return BoolValue { val: true };
    }
    fn get_value(&self) -> String {
        return self.val.to_string();
    }
}

unsafe impl Operable for FloatValue {
    fn add(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" || right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: self.val + right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn subtract(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" || right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: self.val - right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn multiply(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" || right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: self.val * right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn divide(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" || right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: self.val / right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn modulus(&self, right: &impl LSValue) -> Box<&Self> {
        if right.get_value_type() == "Integer" || right.get_value_type() == "Float" {
            return Box::new(&FloatValue {
                val: self.val % right.cast_to_float().val,
            });
        } else {
            todo!("return Error")
        }
    }

    fn and(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }

    fn or(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }
}

struct BoolValue {
    val: bool,
}

impl LSObject for BoolValue {
    fn hash(&self) -> String {
        let sha = Sha256::new();
        sha.update(self.val.to_string());
        return format!("{:X}", sha.finalize());
    }

    fn to_string(&self) -> String {
        return self.val.to_string();
    }

    fn get_type(&self) -> String {
        return String::from("Boolean");
    }

    fn get_name(&self) -> String {
        return format!("BooleanValue({})", &self.val);
    }
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl LSValue for BoolValue {
    fn get_value_type(&self) -> String {
        return self.get_type();
    }

    fn cast_to_int(&self) -> IntValue {
        match self.val {
            true => return IntValue { val: 1 },
            false => return IntValue { val: 0 },
        }
    }

    fn cast_to_float(&self) -> FloatValue {
        match self.val {
            true => return FloatValue { val: 1.0 },
            false => return FloatValue { val: 0.0 },
        }
    }

    fn cast_to_bool(&self) -> BoolValue {
        return *self.clone();
    }
    fn get_value(&self) -> String {
        return self.val.to_string();
    }
}

unsafe impl Operable for BoolValue {
    fn add(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn subtract(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn multiply(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn divide(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn modulus(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn and(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }

    fn or(&self, right: &impl LSValue) -> Box<&Self> {
        todo!()
    }
}

struct UndefinedValue {
    val: String,
}

impl LSObject for UndefinedValue {
    fn hash(&self) -> String {
        let sha = Sha256::new();
        sha.update(self.val.to_string());
        return format!("{:X}", sha.finalize());
    }

    fn to_string(&self) -> String {
        return self.val.to_string();
    }

    fn get_type(&self) -> String {
        return String::from("Undefined");
    }

    fn get_name(&self) -> String {
        return format!("Undefined");
    }
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl LSValue for UndefinedValue {
    fn get_value_type(&self) -> String {
        return String::from("Undefined");
    }

    fn cast_to_int(&self) -> IntValue {
        todo!("Return Error")
    }

    fn cast_to_float(&self) -> FloatValue {
        todo!("Return Error")
    }

    fn cast_to_bool(&self) -> BoolValue {
        todo!("Return Error")
    }
    fn get_value(&self) -> String {
        return self.val.to_string();
    }
}

unsafe impl Operable for UndefinedValue {
    fn add(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn subtract(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn multiply(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn divide(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn modulus(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn and(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn or(&self, right: &impl LSValue) -> Box<&Self> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }
}

struct NullValue {
    val: String,
}

impl LSObject for NullValue {
    fn hash(&self) -> String {
        let sha = Sha256::new();
        sha.update(self.val.to_string());
        return format!("{:X}", sha.finalize());
    }

    fn to_string(&self) -> String {
        return self.val.to_string();
    }

    fn get_type(&self) -> String {
        return String::from("Null");
    }

    fn get_name(&self) -> String {
        return format!("Null");
    }
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}

impl LSValue for NullValue {
    fn get_value_type(&self) -> String {
        return String::from("Null");
    }

    fn cast_to_int(&self) -> IntValue {
        todo!("Return Error")
    }

    fn cast_to_float(&self) -> FloatValue {
        todo!("Return Error")
    }

    fn cast_to_bool(&self) -> BoolValue {
        return BoolValue { val: false };
    }
    fn get_value(&self) -> String {
        return self.val.to_string();
    }
}

unsafe impl Operable for NullValue {
    fn add(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn subtract(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn multiply(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn divide(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn modulus(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&UndefinedValue {
            val: String::from("Undefined"),
        });
    }

    fn and(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&BoolValue {
            val: false && right.cast_to_bool().val,
        });
    }

    fn or(&self, right: &impl LSValue) -> Box<&UndefinedValue> {
        return Box::new(&BoolValue {
            val: false || right.cast_to_bool().val,
        });
    }
}

pub trait ValueOp: LSValue + Operable {}

pub fn token_to_value(left: Token) -> &'static dyn LSValue {
    let int_regx = Regex::new(r"\d+\z").unwrap();
    let float_regx = Regex::new(r"\d+\.\d+\z").unwrap();
    let bool_regx = Regex::new(r"(true\b|false\b)").unwrap();
    let null_regx = Regex::new(r"null\z").unwrap();
    let undef_regx = Regex::new(r"undefined\z").unwrap();
    let left_val = left.tok_value.unwrap().s_val.unwrap();
    if int_regx.is_match(&left_val) {
        return &IntValue {
            val: left_val.parse::<i64>().ok().unwrap(),
        };
    } else if float_regx.is_match(&left_val) {
        return &FloatValue {
            val: left_val.parse::<f64>().ok().unwrap(),
        };
    } else if bool_regx.is_match(&left_val) {
        return &BoolValue {
            val: left_val.parse::<bool>().ok().unwrap(),
        };
    } else if null_regx.is_match(&left_val) {
        return &NullValue {
            val: String::from("Null"),
        };
    } else {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }
}

pub fn token_to_operable(left: Token) -> Box<Operable> {
    let int_regx = Regex::new(r"\d+\z").unwrap();
    let float_regx = Regex::new(r"\d+\.\d+\z").unwrap();
    let bool_regx = Regex::new(r"(true\b|false\b)").unwrap();
    let null_regx = Regex::new(r"null\z").unwrap();
    let undef_regx = Regex::new(r"undefined\z").unwrap();
    let left_val = left.tok_value.unwrap().s_val.unwrap();
    if int_regx.is_match(&left_val) {
        return Box::new(&IntValue {
            val: left_val.parse::<i64>().ok().unwrap(),
        });
    } else if float_regx.is_match(&left_val) {
        return Box::new(&FloatValue {
            val: left_val.parse::<f64>().ok().unwrap(),
        });
    } else if bool_regx.is_match(&left_val) {
        return &BoolValue {
            val: left_val.parse::<bool>().ok().unwrap(),
        };
    } else if null_regx.is_match(&left_val) {
        return &NullValue {
            val: String::from("Null"),
        };
    } else {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }
}

pub fn operate(left: Token, right: Token, operator: Token) -> Token {
    let l = token_to_operable(left);
    let r = token_to_value(right);
    let result;
    match operator.tok_type {
        TokenType::Mult => result = l.multiply(r),
        TokenType::Div => {}
        TokenType::Mod => {}
        TokenType::Plus => {}
        TokenType::Minus => {}
        TokenType::Equals => {}
        TokenType::In => {}
        TokenType::Or => {}
        TokenType::And => {}

        _ => {
            todo!("return error");
        }
    }
    let t_val = TokenValue {
        s_val: Some(result.get_value()),
    };
    return Token {
        tok_type: TokenType::Literal,
        tok_value: Some(t_val),
    };
}
