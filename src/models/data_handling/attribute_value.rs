use std::fmt;

#[derive(Clone, Debug)]
pub enum AttrValue {
    Num(String, f64),
    Cat(String, String),
}

impl fmt::Display for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttrValue::Num(attr, value) => write!(f, "{}={}", attr, value),
            AttrValue::Cat(attr, value) => write!(f, "{}={}", attr, value),
        }
    }
}
