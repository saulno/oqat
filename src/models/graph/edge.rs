use crate::models::data_handling::attribute_values_set::AttributeValuesSetList;

#[derive(Debug, Clone)]
pub enum Edge {
    No(),
    E(usize, usize, AttributeValuesSetList),
}
