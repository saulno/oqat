use std::fmt;

use super::{attribute_value::AttrValue, attribute_values_set::AttributeValuesSetList};

#[derive(Clone, Debug)]
pub struct Row {
    pub class: String,
    pub attributes: AttributeValuesSetList,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.class, self.attributes)?;
        Ok(())
    }
}
