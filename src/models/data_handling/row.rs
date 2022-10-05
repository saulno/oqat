use std::fmt;

use super::attribute_value::AttrValue;

#[derive(Debug)]
pub struct Row {
    pub class: String,
    pub attributes: Vec<AttrValue>,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.class)?;
        for attr in &self.attributes {
            write!(f, "{}, ", attr)?;
        }
        Ok(())
    }
}
