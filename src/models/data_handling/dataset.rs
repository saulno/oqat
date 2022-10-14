use std::{collections::HashSet, fmt};

use ordered_float::OrderedFloat;
use rand::{rngs::StdRng, Rng};

use super::{
    attribute_values_set::{AttributeValuesSet, AttributeValuesSetList},
    row::Row,
};

#[derive(Debug)]
pub struct Dataset {
    pub learning_pos: Vec<Row>,
    pub learning_neg: Vec<Row>,
    pub testing_pos: Vec<Row>,
    pub testing_neg: Vec<Row>,
}

impl fmt::Display for Dataset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Learning set:")?;
        for (idx, row) in self.learning_pos.iter().enumerate() {
            writeln!(f, "  +{} {}", idx, row)?;
        }
        for (idx, row) in self.learning_neg.iter().enumerate() {
            writeln!(f, "  -{} {}", idx, row)?;
        }
        writeln!(f, "Testing set:")?;
        for (idx, row) in self.testing_pos.iter().enumerate() {
            writeln!(f, "  +{} {}", idx, row)?;
        }
        for (idx, row) in self.testing_neg.iter().enumerate() {
            writeln!(f, "  -{} {}", idx, row)?;
        }
        write!(f, "")
    }
}

impl Dataset {
    pub fn new(
        mut rng: StdRng,
        path: &str,
        class_column: &str,
        positive_class: &str,
        learning_frac: usize,
    ) -> Dataset {
        let mut reader = csv::Reader::from_path(path).unwrap();
        let headers = reader.headers().unwrap().clone();

        let class_column_index = headers.iter().position(|x| x == class_column).unwrap();

        let mut all_records = Vec::new();

        for result in reader.records() {
            let record = result.unwrap();
            let class = record.get(class_column_index).unwrap().to_string();

            let mut row = Row {
                class: class.clone(),
                attributes: AttributeValuesSetList::new(),
            };

            for (i, field) in record.iter().enumerate() {
                if i != class_column_index {
                    let attribute = match field.parse::<f64>() {
                        Ok(num) => AttributeValuesSet::Num(
                            headers[i].to_string(),
                            HashSet::from([OrderedFloat(num)]),
                        ),
                        Err(_) => AttributeValuesSet::Cat(
                            headers[i].to_string(),
                            HashSet::from([field.to_string()]),
                        ),
                    };

                    row.attributes.list.push(attribute);
                }
            }

            all_records.push(row);
        }

        let frac = all_records.len() * learning_frac / 100;
        let (mut learning_pos, mut learning_neg, mut testing_pos, mut testing_neg) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for _ in 0..frac {
            let index = rng.gen_range(0..all_records.len());
            let row = all_records.remove(index);

            if row.class == positive_class {
                learning_pos.push(row);
            } else {
                learning_neg.push(row);
            }
        }

        for row in all_records {
            if row.class == positive_class {
                testing_pos.push(row);
            } else {
                testing_neg.push(row);
            }
        }

        Dataset {
            learning_pos,
            learning_neg,
            testing_pos,
            testing_neg,
        }
    }

    // pub fn get_clause_one_learning_negative(&self, idx: usize) -> AttributeValuesSetList {
    //     let mut result = AttributeValuesSetList::new();

    //     for attr_idx in 0..self.learning_neg[idx].attributes.len() {
    //         let mut values = Vec::new();

    //         for row in &self.learning_neg {
    //             values.push(row.attributes[attr_idx].clone());
    //         }

    //         result.list.push(AttributeValuesSetList::from_vec(values));
    //     }

    //     result
    // }
}
