pub mod models;

use rand::rngs::StdRng;
use rand::SeedableRng;
use std::error::Error;

use models::config::Config;

use crate::models::graph::rejectability::create_rejectability_graph;

#[macro_use(c)]
extern crate cute;

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

            let dataset = models::data_handling::dataset::Dataset::new(
                rng.clone(),
                &dataset,
                &class_column,
                &positive_class,
                learning_frac,
            );

            println!("Learning positive len: {}", dataset.learning_pos.len());
            println!("Learning negative len: {}", dataset.learning_neg.len());
            println!("Testing positive len: {}", dataset.testing_pos.len());
            println!("Testing negative len: {}", dataset.testing_neg.len());
            println!("Dataset: {}", dataset);

            let graph = create_rejectability_graph(rng, &dataset);
            println!("Graph: {:?}", graph);
        }
    }

    Ok(())
}
