use std::collections::{HashMap, HashSet};

use rand::{rngs::StdRng, Rng};

use super::data_handling::attribute_values_set::AttributeValuesSetList;

#[derive(Debug)]
pub struct Graph {
    pub adj_mtx: Vec<Vec<Edge>>,
    pub edge_dict: HashMap<usize, HashSet<usize>>,
    pub n_vertex: usize,
    rng: StdRng,
}

#[derive(Debug, Clone)]
pub enum Edge {
    No(),
    E(usize, usize, AttributeValuesSetList),
}

impl Graph {
    pub fn new(rng: StdRng, num_vertex: usize) -> Graph {
        let mut graph = Graph {
            adj_mtx: vec![],
            edge_dict: HashMap::new(),
            n_vertex: 0,
            rng,
        };

        graph.n_vertex = num_vertex;
        graph.adj_mtx = vec![vec![Edge::No(); num_vertex]; num_vertex];

        for i in 0..num_vertex {
            graph.edge_dict.insert(i, HashSet::new());
        }

        graph
    }

    pub fn add_edge(&mut self, u: usize, v: usize, clause_values: &AttributeValuesSetList) {
        self.adj_mtx[u][v] = Edge::E(u, v, clause_values.clone());
        self.adj_mtx[v][u] = Edge::E(v, u, clause_values.clone());

        self.edge_dict.get_mut(&u).unwrap().insert(v);
        self.edge_dict.get_mut(&v).unwrap().insert(u);
    }

    pub fn is_edge(&self, vertex_1: usize, vertex_2: usize) -> bool {
        match self.adj_mtx[vertex_1][vertex_2] {
            Edge::No() => false,
            Edge::E(_, _, _) => true,
        }
    }

    pub fn select_random_vertex(&mut self) -> usize {
        self.rng.gen_range(0..self.n_vertex)
    }

    pub fn get_neighbor_candidates(&self, vertex: usize) -> HashSet<usize> {
        if let Some(set) = self.edge_dict.get(&vertex) {
            set.clone()
        } else {
            HashSet::new()
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use crate::models::data_handling::attribute_values_set::AttributeValuesSet;

    use super::*;

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
}
