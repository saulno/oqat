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

impl Default for AttributeValuesSetList {
    fn default() -> Self {
        Self::new()
    }
}

impl AttributeValuesSetList {
    pub fn new() -> Self {
        AttributeValuesSetList { list: vec![] }
    }

    pub fn from_vec(list: Vec<AttributeValuesSet>) -> Self {
        AttributeValuesSetList { list }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn union(&self, other: &AttributeValuesSetList) -> AttributeValuesSetList {
        let mut result = self.clone();
        for attr_idx in 0..result.list.len() {
            match &result.list[attr_idx] {
                AttributeValuesSet::Num(attr_name, current_values_set) => {
                    if let AttributeValuesSet::Num(_, new_values_set) = &other.list[attr_idx] {
                        let union_values_set: HashSet<OrderedFloat<f64>> =
                            current_values_set.union(new_values_set).cloned().collect();
                        result.list[attr_idx] =
                            AttributeValuesSet::Num(attr_name.clone(), union_values_set);
                    }
                }
                AttributeValuesSet::Cat(attr_name, current_values_set) => {
                    if let AttributeValuesSet::Cat(_, new_values_set) = &other.list[attr_idx] {
                        let union_values_set: HashSet<String> =
                            current_values_set.union(new_values_set).cloned().collect();
                        result.list[attr_idx] =
                            AttributeValuesSet::Cat(attr_name.clone(), union_values_set);
                    }
                }
                AttributeValuesSet::Empty => {
                    result.list[attr_idx] = other.list[attr_idx].clone();
                }
            }
        }

        result
    }

    pub fn intersection(&self, other: &AttributeValuesSetList) -> AttributeValuesSetList {
        let mut result = self.clone();
        for attr_idx in 0..result.list.len() {
            match &result.list[attr_idx] {
                AttributeValuesSet::Num(attr_name, current_values_set) => {
                    if let AttributeValuesSet::Num(_, new_values_set) = &other.list[attr_idx] {
                        let intersection_values_set: HashSet<OrderedFloat<f64>> =
                            current_values_set
                                .intersection(new_values_set)
                                .cloned()
                                .collect();
                        result.list[attr_idx] =
                            AttributeValuesSet::Num(attr_name.clone(), intersection_values_set);
                    }
                }
                AttributeValuesSet::Cat(attr_name, current_values_set) => {
                    if let AttributeValuesSet::Cat(_, new_values_set) = &other.list[attr_idx] {
                        let intersection_values_set: HashSet<String> = current_values_set
                            .intersection(new_values_set)
                            .cloned()
                            .collect();
                        result.list[attr_idx] =
                            AttributeValuesSet::Cat(attr_name.clone(), intersection_values_set);
                    }
                }
                AttributeValuesSet::Empty => {
                    result.list[attr_idx] = other.list[attr_idx].clone();
                }
            }
        }

        result
    }
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
