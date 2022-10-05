use std::{collections::HashSet, fmt};

use ordered_float::OrderedFloat;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeValuesSet {
    // First element is attribute name, second is a set of values
    Num(String, HashSet<OrderedFloat<f64>>),
    Cat(String, HashSet<String>),
    Empty,
}

impl fmt::Display for AttributeValuesSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttributeValuesSet::Num(attr, values) => {
                write!(f, "{}: {{", attr)?;
                for value in values {
                    write!(f, "{}, ", value)?;
                }
                write!(f, "}}")
            }
            AttributeValuesSet::Cat(attr, values) => {
                write!(f, "{}: {{", attr)?;
                for value in values {
                    write!(f, "{}, ", value)?;
                }
                write!(f, "}}")
            }
            AttributeValuesSet::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttributeValuesSetList {
    pub list: Vec<AttributeValuesSet>,
}

impl fmt::Display for AttributeValuesSetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for set in &self.list {
            write!(f, "{}", set)?;
            if set != self.list.last().unwrap() {
                write!(f, ", ")?;
            }
        }
        write!(f, " ]")?;
        Ok(())
    }
}
