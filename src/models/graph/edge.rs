use crate::models::data_handling::attribute_values_set::AttributeValuesSetList;

// pub enum Edge {
//     No(),
//     E(usize, usize, AttributeValuesSetList),
// }

pub type Edge = Option<AttributeValuesSetList>;
