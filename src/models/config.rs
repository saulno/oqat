// Enum with the different options to run

use super::ant_colony_optimization::aco_parameters::{ACOAlgorithm, ACOParameters};

#[derive(Debug)]
pub enum Config {
    Help(),
    Run(String, String, String, usize, Box<Algorithm>),
}

#[derive(Debug)]
pub enum Algorithm {
    Ants(ACOAlgorithm, ACOParameters),
}

impl Config {
    // Create new config
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        match args[1].as_str() {
            "help" | "-h" | "--help" => Ok(Config::Help()),
            "run" => {
                if args.len() < 7 {
                    return Err("Not enough arguments");
                }

                let dataset = args[2][3..].to_string();
                let class_column = args[3][10..].to_string();
                let positive_class = args[4][8..].to_string();
                let learning_frac = args[5][7..].parse::<usize>().unwrap();
                let algorithm = args[6][5..].to_string();

                match algorithm.as_str() {
                    "vertex-ac" | "edge-ac" => {
                        if args.len() < 13 {
                            return Err("Not enough arguments");
                        }

                        let ants = args[7][5..].parse::<usize>().unwrap();
                        let gen = args[8][4..].parse::<usize>().unwrap();
                        let alpha = args[9][6..].parse::<f64>().unwrap();
                        let rho = args[10][4..].parse::<f64>().unwrap();
                        let tau_max = args[11][8..].parse::<f64>().unwrap();
                        let tau_min = args[12][8..].parse::<f64>().unwrap();

                        let algo = Algorithm::Ants(
                            if algorithm.as_str() == "vertex-ac" {
                                ACOAlgorithm::VertexAC
                            } else {
                                ACOAlgorithm::EdgeAC
                            },
                            ACOParameters::new(gen, ants, alpha, rho, tau_max, tau_min),
                        );

                        Ok(Config::Run(
                            dataset,
                            class_column,
                            positive_class,
                            learning_frac,
                            Box::new(algo),
                        ))
                    }
                    _ => Err("Algorithm not found"),
                }
            }
            _ => Err("Invalid argument"),
        }
    }
}
