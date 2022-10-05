use std::collections::{HashMap, HashSet};

use rand::{rngs::StdRng, Rng};

use crate::models::data_handling::attribute_values_set::AttributeValuesSetList;

use super::edge::Edge;

#[derive(Debug)]
pub struct Graph {
    pub adj_mtx: Vec<Vec<Edge>>,
    pub edge_dict: HashMap<usize, HashSet<usize>>,
    pub n_vertex: usize,
    rng: StdRng,
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
