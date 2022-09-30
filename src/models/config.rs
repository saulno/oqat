// Enum with the different options to run

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    Help(),
    Run(String, String, String, usize, Algorithm),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Algorithm {
    Ants(usize, usize),
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
                    "aco" => {
                        if args.len() < 9 {
                            return Err("Not enough arguments");
                        }

                        let ants = args[7][5..].parse::<usize>().unwrap();
                        let gen = args[8][4..].parse::<usize>().unwrap();

                        Ok(Config::Run(
                            dataset,
                            class_column,
                            positive_class,
                            learning_frac,
                            Algorithm::Ants(ants, gen),
                        ))
                    }
                    _ => Err("Algorithm not found"),
                }
            }
            _ => Err("Invalid argument"),
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Config::new(&vec!["".to_string(), "help".to_string()]).unwrap(),
            Config::Help()
        );
        assert_eq!(
            Config::new(&vec!["".to_string(), "-h".to_string()]).unwrap(),
            Config::Help()
        );
        assert_eq!(
            Config::new(&vec!["".to_string(), "--help".to_string()]).unwrap(),
            Config::Help()
        );

        let args =
            "oqat run ds=./test1.csv classname=class concept=yes learn%=80 algo=aco ants=10 gen=3";
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
        assert_eq!(
            Config::new(&args[..]).unwrap(),
            Config::Run(
                "./test1.csv".to_string(),
                "class".to_string(),
                "yes".to_string(),
                80,
                Algorithm::Ants(10, 3)
            )
        );
    }
}
