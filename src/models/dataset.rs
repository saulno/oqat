use rand::Rng;

// Build a dataset forma acsv file
//
// # Arguments
//
// * `path` - The path to the csv file
// * `class_column` - The name of the column that contains the class
// * `positive_class` - The name of the positive class
// * `learning_frac` - The fraction of the dataset that will be used for learning
//
// # Returns
//
// A dataset

#[derive(Debug)]
pub struct Dataset {
    pub learning_pos: Vec<Row>,
    pub learning_neg: Vec<Row>,
    pub testing_pos: Vec<Row>,
    pub testing_neg: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    pub class: String,
    pub features: Vec<Op>,
}

#[derive(Debug)]
pub enum Op {
    Num(String, f64),
    Str(String, String),
}

impl Dataset {
    pub fn new(
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
                features: Vec::new(),
            };

            for (i, field) in record.iter().enumerate() {
                if i != class_column_index {
                    let feature = match field.parse::<f64>() {
                        Ok(num) => Op::Num(headers[i].to_string(), num),
                        Err(_) => Op::Str(headers[i].to_string(), field.to_string()),
                    };

                    row.features.push(feature);
                }
            }

            all_records.push(row);
        }

        let mut rng = rand::thread_rng();
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

    #[test]
    fn test_new() {
        let dataset = Dataset::new("datasets/test1.csv", "class", "yes", 80);

        let len = dataset.learning_pos.len()
            + dataset.learning_neg.len()
            + dataset.testing_pos.len()
            + dataset.testing_neg.len();
        assert_eq!(len, 9);
    }
}
