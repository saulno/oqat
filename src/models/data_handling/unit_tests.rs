// tests
#[cfg(test)]
mod tests {
    use crate::models::data_handling::dataset::Dataset;

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
