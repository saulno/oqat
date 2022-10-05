use std::fmt;

use rand::{rngs::StdRng, Rng};

use super::{attribute_value::AttrValue, row::Row};

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
        for row in &self.learning_pos {
            writeln!(f, "  + {}", row)?;
        }
        for row in &self.learning_neg {
            writeln!(f, "  - {}", row)?;
        }
        writeln!(f, "Testing set:")?;
        for row in &self.testing_pos {
            writeln!(f, "  + {}", row)?;
        }
        for row in &self.testing_neg {
            writeln!(f, "  - {}", row)?;
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
                attributes: Vec::new(),
            };

            for (i, field) in record.iter().enumerate() {
                if i != class_column_index {
                    let attribute = match field.parse::<f64>() {
                        Ok(num) => AttrValue::Num(headers[i].to_string(), num),
                        Err(_) => AttrValue::Cat(headers[i].to_string(), field.to_string()),
                    };

                    row.attributes.push(attribute);
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
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_new() {
        let rng = StdRng::seed_from_u64(1000);
        let dataset = Dataset::new(rng, "datasets/test1.csv", "class", "yes", 80);

        let len = dataset.learning_pos.len()
            + dataset.learning_neg.len()
            + dataset.testing_pos.len()
            + dataset.testing_neg.len();
        assert_eq!(len, 9);

        assert_eq!(dataset.learning_pos.len(), 3);
        assert_eq!(dataset.learning_neg.len(), 4);
        assert_eq!(dataset.testing_pos.len(), 1);
        assert_eq!(dataset.testing_neg.len(), 1);
    }
}
