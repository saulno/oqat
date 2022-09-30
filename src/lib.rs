pub mod models;

use std::error::Error;
use rand::SeedableRng;
use rand::rngs::StdRng;

use models::config::Config;

const SEED: u64 = 1000;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Help() => println!("Help"),
        Config::Run(dataset, class_column, positive_class, learning_frac, algorithm) => {
            println!("Run");
            println!("Dataset: {}", dataset);
            println!("Class column: {}", class_column);
            println!("Positive class: {}", positive_class);
            println!("Learning fraction: {}", learning_frac);
            println!("Algorithm: {:?}", algorithm);

            let rng = StdRng::seed_from_u64(SEED);

            let dataset = models::dataset::Dataset::new(
                rng, 
                &dataset,
                &class_column,
                &positive_class,
                learning_frac,
            );

            println!("Learning positive len: {}", dataset.learning_pos.len());
            println!("Learning negative len: {}", dataset.learning_neg.len());
            println!("Testing positive len: {}", dataset.testing_pos.len());
            println!("Testing negative len: {}", dataset.testing_neg.len());
            println!("Dataset: {:?}", dataset);
        }
    }

    Ok(())
}
