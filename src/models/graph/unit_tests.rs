// tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::models::{
        data_handling::{
            attribute_values_set::{AttributeValuesSet, AttributeValuesSetList},
            dataset::Dataset,
        },
        graph::{
            rejectability::{
                construct_attribute_sets, create_rejectability_graph, exists_clause_one_positive,
                find_clause_one_positive,
            },
            rejectability_graph::Graph,
        },
    };

    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_graph_new() {
        let rng = StdRng::seed_from_u64(1000);
        let mut graph = Graph::new(rng, 5);

        let clause_values = AttributeValuesSetList {
            list: vec![
                AttributeValuesSet::Num("color".to_string(), HashSet::new()),
                AttributeValuesSet::Cat("size".to_string(), HashSet::new()),
            ],
        };

        graph.add_edge(0, 1, &clause_values.clone());
        graph.add_edge(0, 3, &clause_values.clone());
        graph.add_edge(1, 3, &clause_values.clone());
        graph.add_edge(1, 4, &clause_values.clone());
        graph.add_edge(2, 4, &clause_values.clone());

        assert_eq!(graph.is_edge(0, 1), true);
        assert_eq!(graph.is_edge(0, 3), true);
        assert_eq!(graph.is_edge(1, 3), true);
        assert_eq!(graph.is_edge(1, 4), true);
        assert_eq!(graph.is_edge(2, 4), true);

        assert_eq!(graph.is_edge(1, 0), true);
        assert_eq!(graph.is_edge(3, 0), true);
        assert_eq!(graph.is_edge(3, 1), true);
        assert_eq!(graph.is_edge(4, 1), true);
        assert_eq!(graph.is_edge(4, 2), true);

        assert_eq!(graph.is_edge(0, 0), false);
        assert_eq!(graph.is_edge(1, 1), false);

        assert_eq!(graph.edge_dict.get(&0).unwrap().len(), 2);
        assert_eq!(graph.edge_dict.get(&0).unwrap(), &HashSet::from([1, 3]));
    }

    #[test]
    fn test_graph_select_random_vertex() {
        let rng = StdRng::seed_from_u64(1000);
        let mut graph = Graph::new(rng, 5);

        let mut random_vertex = graph.select_random_vertex();
        let options = HashSet::from([0, 1, 2, 3, 4]);
        assert!(options.contains(&random_vertex));

        random_vertex = graph.select_random_vertex();
        assert!(options.contains(&random_vertex));

        random_vertex = graph.select_random_vertex();
        assert!(options.contains(&random_vertex));
    }

    #[test]
    fn test_graph_get_neighbor_candidates() {
        let rng = StdRng::seed_from_u64(1000);
        let mut graph = Graph::new(rng, 5);

        let clause_values = AttributeValuesSetList {
            list: vec![
                AttributeValuesSet::Num("color".to_string(), HashSet::new()),
                AttributeValuesSet::Cat("size".to_string(), HashSet::new()),
            ],
        };

        graph.add_edge(0, 1, &clause_values.clone());
        graph.add_edge(0, 2, &clause_values.clone());
        graph.add_edge(0, 3, &clause_values.clone());
        graph.add_edge(1, 3, &clause_values.clone());
        graph.add_edge(1, 4, &clause_values.clone());
        graph.add_edge(2, 3, &clause_values.clone());
        graph.add_edge(2, 4, &clause_values.clone());
        graph.add_edge(3, 4, &clause_values.clone());

        let neighbor_candidates = graph.get_neighbor_candidates(0);
        assert_eq!(neighbor_candidates.len(), 3);
        assert_eq!(neighbor_candidates, HashSet::from([1, 2, 3]));

        let neighbor_candidates = graph.get_neighbor_candidates(1);
        assert_eq!(neighbor_candidates.len(), 3);
        assert_eq!(neighbor_candidates, HashSet::from([0, 3, 4]));

        let neighbor_candidates = graph.get_neighbor_candidates(2);
        assert_eq!(neighbor_candidates.len(), 3);
        assert_eq!(neighbor_candidates, HashSet::from([0, 3, 4]));

        let neighbor_candidates = graph.get_neighbor_candidates(3);
        assert_eq!(neighbor_candidates.len(), 4);
        assert_eq!(neighbor_candidates, HashSet::from([0, 1, 2, 4]));

        let neighbor_candidates = graph.get_neighbor_candidates(4);
        assert_eq!(neighbor_candidates.len(), 3);
        assert_eq!(neighbor_candidates, HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_create_rejectability_graph() {
        let rng = StdRng::seed_from_u64(1000);
        let dataset = Dataset::new(rng, "datasets/test1.csv", "class", "yes", 80);

        let rng = StdRng::seed_from_u64(1000);
        let graph = create_rejectability_graph(rng, &dataset);

        assert_eq!(graph.n_vertex, 4);
    }

    #[test]
    fn test_construct_attribute_sets() {
        let rng = StdRng::seed_from_u64(1000);
        let dataset = Dataset::new(rng, "datasets/test1.csv", "class", "yes", 80);

        assert_eq!(dataset.learning_neg.len(), 4);
        let subset = [0, 1, 2];
        let values = construct_attribute_sets(&dataset.learning_neg, &subset);

        assert_eq!(values.list.len(), 2);

        let mut hs: HashSet<String> = HashSet::new();
        hs.insert("small".to_string());
        hs.insert("large".to_string());
        hs.insert("medium".to_string());
        assert_eq!(
            values.list[0],
            AttributeValuesSet::Cat("size".to_string(), hs)
        );

        let mut hs: HashSet<String> = HashSet::new();
        hs.insert("green".to_string());
        hs.insert("red".to_string());
        assert_eq!(
            values.list[1],
            AttributeValuesSet::Cat("color".to_string(), hs)
        );
    }

    #[test]
    fn test_exists_clause_one_positive() {
        let rng = StdRng::seed_from_u64(1000);
        let dataset = Dataset::new(rng, "datasets/test1.csv", "class", "yes", 80);

        let positive_idx = 0;

        let negative_pair_attrs = construct_attribute_sets(&dataset.learning_neg, &[0, 1]);
        let exists_clause =
            exists_clause_one_positive(&dataset.learning_pos[positive_idx], &negative_pair_attrs);
        assert_eq!(exists_clause, true);

        let negative_pair_attrs = construct_attribute_sets(&dataset.learning_neg, &[1, 2]);
        let exists_clause =
            exists_clause_one_positive(&dataset.learning_pos[positive_idx], &negative_pair_attrs);
        assert_eq!(exists_clause, false);
    }

    #[test]
    fn test_find_clause_one_positive() {
        let rng = StdRng::seed_from_u64(1000);
        let dataset = Dataset::new(rng, "datasets/test1.csv", "class", "yes", 80);

        let positive_idx = 0;

        let negative_pair_attrs = construct_attribute_sets(&dataset.learning_neg, &[0, 1]);
        let clause =
            find_clause_one_positive(&dataset.learning_pos, positive_idx, &negative_pair_attrs);
        assert_eq!(clause.list.len(), 2);
        assert_eq!(
            clause.list[0],
            AttributeValuesSet::Cat("size".to_string(), HashSet::new())
        );
        assert_eq!(
            clause.list[1],
            AttributeValuesSet::Cat("color".to_string(), HashSet::from(["red".to_string()]))
        );

        let negative_pair_attrs = construct_attribute_sets(&dataset.learning_neg, &[1, 2]);
        let clause =
            find_clause_one_positive(&dataset.learning_pos, positive_idx, &negative_pair_attrs);
        assert_eq!(clause.list.len(), 2);
        assert_eq!(
            clause.list[0],
            AttributeValuesSet::Cat("size".to_string(), HashSet::new())
        );
        assert_eq!(
            clause.list[1],
            AttributeValuesSet::Cat("color".to_string(), HashSet::new())
        );
    }
}
