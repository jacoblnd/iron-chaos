use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: usize,
    input_ids: Vec<usize>,
    truth_table: HashMap<Vec<u8>, bool>,
    state: bool,
}

#[derive(Debug)]
pub struct SynchronousRBN<R: RandProvider = RandRBN> {
    n: usize,
    k: usize,
    p: f64,
    nodes: Vec<Node>,
    random_provider: R,
}

/// Defines the common interface for random generation behavior used by
/// RBNs.
pub trait RandProvider {
    /// Return a random boolean based on probability p.
    fn random_bool(&self, p: f64) -> bool;
    /// Return K distinct random values in range 0..n.
    fn random_distinct(&self, k: usize, n: usize) -> Vec<usize>;
}

pub struct RandRBN {}

impl RandRBN {
    fn new() -> Self {
        Self {}
    }
}

impl RandProvider for RandRBN {
    /// Returns a boolean with probability p that it will be true.
    fn random_bool(&self, p: f64) -> bool {
        let mut rng = rand::rng();
        match rng.random::<f64>() < p {
            true => true,
            false => false,
        }
    }
    /// Returns an array of K distinct usize between 0..n.
    /// Will panic if K > n.
    fn random_distinct(&self, k: usize, n: usize) -> Vec<usize> {
        let mut rng = rand::rng();
        rand::seq::index::sample(&mut rng, n, k).into_vec()
    }
}

pub trait RBN {
    fn setup_nodes(&mut self);
    fn rand_activate_nodes(&mut self, activate_probability: f64);
    fn advance(&mut self, t: u32) -> Vec<u8>;
}

impl SynchronousRBN {
    /// Helper for easily initializing a SynchronousRBN.
    pub fn new(n: usize, k: usize, p: f64) -> Self {
        Self {
            n,
            k,
            p,
            nodes: Vec::new(),
            random_provider: RandRBN::new(),
        }
    }
}

impl<R: RandProvider> RBN for SynchronousRBN<R> {
    fn setup_nodes(&mut self) {
        // Create nodes with ids: 0..n.
        for i in 0..self.n {
            let node = Node {
                id: i as usize,
                input_ids: self.random_provider.random_distinct(self.k, self.n),
                truth_table: generate_truth_table(self.k, self.p, &self.random_provider),
                state: false,
            };
            self.nodes.push(node);
        }
    }
    fn rand_activate_nodes(&mut self, activate_probability: f64) {
        for node in self.nodes.iter_mut() {
            node.state = self.random_provider.random_bool(activate_probability);
        }
    }
    fn advance(&mut self, t: u32) -> Vec<u8> {
        // Grab the vec of input_ids for each node.
        let input_id_vec: Vec<Vec<usize>> = self
            .nodes
            .iter()
            .map(|node| node.input_ids.clone())
            .collect();
        // Grab the state from the node in each input.
        let input_states: Vec<Vec<u8>> = input_id_vec
            .iter()
            .map(|input_ids| {
                input_ids
                    .iter()
                    .map(|input_id| self.nodes[*input_id].state as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();
        // Set the state of each node based on the input.
        let mut output_vec: Vec<u8> = Vec::with_capacity(self.nodes.len());
        for (index, node) in self.nodes.iter_mut().enumerate() {
            let output = *node.truth_table.get(&input_states[index]).unwrap();
            node.state = output;
            match output {
                true => output_vec.push(u8::from(1)),
                false => output_vec.push(u8::from(0)),
            }
        }
        output_vec
    }
}

/// Generate a random truth table of n inputs with probability p a given input combination
/// will return true.
fn generate_truth_table<R: RandProvider>(
    n: usize,
    p: f64,
    random_provider: &R,
) -> HashMap<Vec<u8>, bool> {
    let num_rows = 2_usize.pow(n as u32);

    let mut truth_table = HashMap::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut input_vector = Vec::with_capacity(n);
        for j in (0..n).rev() {
            input_vector.push(((i >> j) & 1) as u8);
        }

        truth_table.insert(input_vector, random_provider.random_bool(p));
    }
    truth_table
}

#[cfg(test)]
#[path = "unit_tests/rbn.rs"]
mod tests;
