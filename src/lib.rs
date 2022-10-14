pub mod models;

use rand::rngs::StdRng;
use rand::SeedableRng;
use std::error::Error;

use models::config::Config;

use crate::models::ant_colony_optimization::aco::ACO;
use crate::models::ant_colony_optimization::aco_parameters;
use crate::models::ant_colony_optimization::edge_ac::EdgeAC;
use crate::models::ant_colony_optimization::vertex_ac::VertexAC;
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

            let graph = create_rejectability_graph(rng.clone(), &dataset);

            println!(
                "Rejectability graph created. Number of nodes: {}",
                graph.n_vertex
            );

            match *algorithm {
                models::config::Algorithm::Ants(aco_algo, mut aco_parameters) => {
                    aco_parameters.graph = graph;
                    aco_parameters.rand = rng;
                    match aco_algo {
                        aco_parameters::ACOAlgorithm::VertexAC => {
                            let mut vertex_ac = VertexAC::new(&aco_parameters);
                            while !aco_parameters.graph.available_vertex.is_empty() {
                                let best_clique = vertex_ac.aco_procedure(&mut aco_parameters);
                                aco_parameters
                                    .graph
                                    .remove_vertex_set_from_available(&best_clique);
                                println!("Best clique: |{}| {:?}", best_clique.len(), best_clique);
                                println!(
                                    "Clique clause: {}",
                                    &aco_parameters.graph.get_clique_clause(best_clique)
                                );
                            }
                        }
                        aco_parameters::ACOAlgorithm::EdgeAC => {
                            let mut edge_ac = EdgeAC::new(&aco_parameters);
                            while !aco_parameters.graph.available_vertex.is_empty() {
                                let best_clique = edge_ac.aco_procedure(&mut aco_parameters);
                                aco_parameters
                                    .graph
                                    .remove_vertex_set_from_available(&best_clique);
                                println!("Best clique: |{}| {:?}", best_clique.len(), best_clique);
                                println!(
                                    "Clique clause: {}",
                                    &aco_parameters.graph.get_clique_clause(best_clique)
                                );
                            }
                        }
                    };
                }
            };
        }
    }

    Ok(())
}
