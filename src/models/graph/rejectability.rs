use ordered_float::OrderedFloat;
use rand::rngs::StdRng;
use std::collections::HashSet;

use super::{
    super::data_handling::{
        attribute_value::AttrValue,
        attribute_values_set::{AttributeValuesSet, AttributeValuesSetList},
        dataset::Dataset,
        row::Row,
    },
    rejectability_graph::Graph,
};

// create rejectability graph
pub fn create_rejectability_graph(rng: StdRng, dataset: &Dataset) -> Graph {
    let mut graph = Graph::new(rng, dataset.learning_neg.len());

    // add an edge for every ossible pair of negative examples
    for i in 0..dataset.learning_neg.len() {
        for j in i + 1..dataset.learning_neg.len() {
            // get a list of sets with every selector of the two negative examples
            let negative_pair_attrs = construct_attribute_sets(&dataset.learning_neg, &[i, j]);

            // clause is a list of sets containing every selector
            // that is present in every positive element and
            // not in the two negative elements
            let mut clause: AttributeValuesSetList = AttributeValuesSetList {
                list: c![AttributeValuesSet::Empty, for _i in 0..dataset.learning_pos[0].attributes.len()],
            };
            let mut exists_clause_for_all_positive = true;

            // check every element in the positive dataset to see if theres a complete clause tha rejects the pair
            for (positive_idx, positive) in dataset.learning_pos.iter().enumerate() {
                let exists_clause_two_neg_on_pos =
                    exists_clause_one_positive(positive, &negative_pair_attrs);

                if !exists_clause_two_neg_on_pos {
                    exists_clause_for_all_positive = false;
                    break;
                }

                // find the clause that rejects the pair and accepts current positive element
                let singular_clause_two_neg_one_pos = find_clause_one_positive(
                    &dataset.learning_pos,
                    positive_idx,
                    &negative_pair_attrs,
                );

                // add to the clause, the new selectors for this positive element
                clause = update_clause(&clause, &singular_clause_two_neg_one_pos);

                exists_clause_for_all_positive =
                    exists_clause_for_all_positive && exists_clause_two_neg_on_pos;
            }

            if exists_clause_for_all_positive {
                graph.add_edge(i, j, &clause);
                println!(
                    "There's an edge between {} and {}, with clause {}",
                    i, j, clause
                );
            }
        }
    }

    graph
}

pub fn update_clause(
    old_clause: &AttributeValuesSetList,
    new_attributes: &AttributeValuesSetList,
) -> AttributeValuesSetList {
    let mut clause = old_clause.clone();
    for attr_idx in 0..clause.list.len() {
        match &clause.list[attr_idx] {
            AttributeValuesSet::Num(attr_name, current_values_set) => {
                if let AttributeValuesSet::Num(_, new_values_set) = &new_attributes.list[attr_idx] {
                    let union_values_set: HashSet<OrderedFloat<f64>> =
                        current_values_set.union(new_values_set).cloned().collect();
                    clause.list[attr_idx] =
                        AttributeValuesSet::Num(attr_name.clone(), union_values_set);
                }
            }
            AttributeValuesSet::Cat(attr_name, current_values_set) => {
                if let AttributeValuesSet::Cat(_, new_values_set) = &new_attributes.list[attr_idx] {
                    let union_values_set: HashSet<String> =
                        current_values_set.union(new_values_set).cloned().collect();
                    clause.list[attr_idx] =
                        AttributeValuesSet::Cat(attr_name.clone(), union_values_set);
                }
            }
            AttributeValuesSet::Empty => {
                let new_clause = new_attributes.list[attr_idx].clone();
                if new_clause != AttributeValuesSet::Empty {
                    clause.list[attr_idx] = new_clause;
                }
            }
        }
    }

    clause
}

pub fn exists_clause_one_positive(
    positive: &Row,
    negative_pair_attrs: &AttributeValuesSetList,
) -> bool {
    let mut exists_clause = false;

    for (pos_attr_idx, pos_attr) in positive.attributes.iter().enumerate() {
        match &negative_pair_attrs.list[pos_attr_idx] {
            AttributeValuesSet::Num(_, neg_values_set) => {
                if let AttrValue::Num(_, pos_value) = pos_attr {
                    exists_clause =
                        exists_clause || !neg_values_set.contains(&OrderedFloat(*pos_value));
                }
            }
            AttributeValuesSet::Cat(_, neg_values_set) => {
                if let AttrValue::Cat(_, pos_value) = pos_attr {
                    exists_clause = exists_clause || !neg_values_set.contains(pos_value);
                }
            }
            AttributeValuesSet::Empty => continue,
        }
    }

    exists_clause
}

pub fn find_clause_one_positive(
    positive_dataset: &[Row],
    positive_idx: usize,
    negative_pair_attrs: &AttributeValuesSetList,
) -> AttributeValuesSetList {
    let positive_element_attrs = construct_attribute_sets(positive_dataset, &[positive_idx]);
    let mut clause: AttributeValuesSetList = AttributeValuesSetList {
        list: c![AttributeValuesSet::Empty, for _i in 0..positive_dataset[0].attributes.len()],
    };

    for (pos_attr_idx, pos_attr) in positive_element_attrs.list.iter().enumerate() {
        match pos_attr {
            AttributeValuesSet::Num(_, values_set) => {
                if let AttributeValuesSet::Num(attr_name, neg_values_set) =
                    &negative_pair_attrs.list[pos_attr_idx]
                {
                    clause.list[pos_attr_idx] = AttributeValuesSet::Num(
                        attr_name.clone(),
                        values_set.difference(neg_values_set).cloned().collect(),
                    );
                }
            }
            AttributeValuesSet::Cat(_, values_set) => {
                if let AttributeValuesSet::Cat(attr_name, neg_values_set) =
                    &negative_pair_attrs.list[pos_attr_idx]
                {
                    clause.list[pos_attr_idx] = AttributeValuesSet::Cat(
                        attr_name.clone(),
                        values_set.difference(neg_values_set).cloned().collect(),
                    );
                }
            }
            AttributeValuesSet::Empty => continue,
        }
    }

    clause
}

// construct a list of sets containig the values of every atrribute for each element in a subest of the dataset
pub fn construct_attribute_sets(dataset: &[Row], subset: &[usize]) -> AttributeValuesSetList {
    let subset_elements = subset
        .iter()
        .map(|&i| dataset[i].attributes.clone())
        .collect::<Vec<_>>();

    let values: Vec<AttributeValuesSet> = subset_elements[0]
        .iter()
        .map(|attr| match attr {
            AttrValue::Num(name, _) => AttributeValuesSet::Num(name.clone(), HashSet::new()),
            AttrValue::Cat(name, _) => AttributeValuesSet::Cat(name.clone(), HashSet::new()),
        })
        .collect();
    let mut values: AttributeValuesSetList = AttributeValuesSetList { list: values };

    for elem in subset_elements {
        for (idx, attr) in elem.iter().enumerate() {
            match attr {
                AttrValue::Num(_, value) => {
                    if let AttributeValuesSet::Num(_, set) = &mut values.list[idx] {
                        set.insert(OrderedFloat(*value));
                    }
                }
                AttrValue::Cat(_, value) => {
                    if let AttributeValuesSet::Cat(_, set) = &mut values.list[idx] {
                        set.insert(value.clone());
                    }
                }
            }
        }
    }

    values
}
