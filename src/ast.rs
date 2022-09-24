use std::any::Any;

use crate::token::Token;
use crate::token::TokenType;
use sha2::Digest;
use sha2::Sha256;

pub trait LSObject {
    fn hash(&self) -> String;
    fn to_string(&self) -> String;
    fn get_type(&self) -> String;
    fn get_name(&self) -> String;
}
pub trait LSValue {
    fn get_value_type(&self) -> String;
    fn cast_to_int(&self) -> IntValue;
    fn cast_to_float(&self) -> FloatValue;
    fn cast_to_bool(&self) -> BoolValue;
}

trait Operable {
    fn add(&self, num: &impl LSValue) -> &dyn LSValue;
    fn subtract(&self, num: &impl LSValue) -> &dyn LSValue;
    fn multiply(&self, num: &impl LSValue) -> &dyn LSValue;
    fn divide(&self, num: &impl LSValue) -> &dyn LSValue;
    fn modulus(&self, num: &impl LSValue) -> &dyn LSValue;
    fn and(&self, right: &impl LSValue) -> &dyn LSValue;
    fn or(&self, right: &impl LSValue) -> &dyn LSValue;
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
}

impl Operable for IntValue {
    fn add(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" {
            return &IntValue {
                val: self.val + num.cast_to_int().val,
            };
        } else if num.get_value_type() == "Float" {
            return &FloatValue {
                val: (self.val as f64) + num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn subtract(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" {
            return &IntValue {
                val: self.val - num.cast_to_int().val,
            };
        } else if num.get_value_type() == "Float" {
            return &FloatValue {
                val: (self.val as f64) - num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn multiply(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" {
            return &IntValue {
                val: self.val * num.cast_to_int().val,
            };
        } else if num.get_value_type() == "Float" {
            return &FloatValue {
                val: (self.val as f64) * num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn divide(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" {
            return &IntValue {
                val: self.val / num.cast_to_int().val,
            };
        } else if num.get_value_type() == "Float" {
            return &FloatValue {
                val: (self.val as f64) / num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn modulus(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" {
            return &IntValue {
                val: self.val % num.cast_to_int().val,
            };
        } else if num.get_value_type() == "Float" {
            return &FloatValue {
                val: (self.val as f64) % num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn and(&self, right: &impl LSValue) -> &dyn LSValue {
        todo!()
    }

    fn or(&self, right: &impl LSValue) -> &dyn LSValue {
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
}

impl Operable for FloatValue {
    fn add(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" || num.get_value_type() == "Float" {
            return &FloatValue {
                val: self.val + num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn subtract(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" || num.get_value_type() == "Float" {
            return &FloatValue {
                val: self.val - num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn multiply(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" || num.get_value_type() == "Float" {
            return &FloatValue {
                val: self.val * num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn divide(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" || num.get_value_type() == "Float" {
            return &FloatValue {
                val: self.val / num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn modulus(&self, num: &impl LSValue) -> &dyn LSValue {
        if num.get_value_type() == "Integer" || num.get_value_type() == "Float" {
            return &FloatValue {
                val: self.val % num.cast_to_float().val,
            };
        } else {
            todo!("return Error")
        }
    }

    fn and(&self, right: &impl LSValue) -> &dyn LSValue {
        todo!()
    }

    fn or(&self, right: &impl LSValue) -> &dyn LSValue {
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
}

impl Operable for UndefinedValue {
    fn add(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn subtract(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn multiply(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn divide(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn modulus(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn and(&self, right: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn or(&self, right: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
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
}

impl Operable for NullValue {
    fn add(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn subtract(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn multiply(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn divide(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn modulus(&self, num: &impl LSValue) -> &dyn LSValue {
        return &UndefinedValue {
            val: String::from("Undefined"),
        };
    }

    fn and(&self, right: &impl LSValue) -> &dyn LSValue {
        return &BoolValue {
            val: false && right.cast_to_bool().val,
        };
    }

    fn or(&self, right: &impl LSValue) -> &dyn LSValue {
        return &BoolValue {
            val: false || right.cast_to_bool().val,
        };
    }
}
