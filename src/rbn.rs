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
pub struct SynchronousRBN {
    nodes: Vec<Node>,
}

pub trait RBN {
    fn advance(&mut self, t: u32) -> Vec<u8>;
    fn rand_activate(&mut self, p: f64);
}

impl RBN for SynchronousRBN {
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
    fn rand_activate(&mut self, p: f64) {
        let mut rng = rand::rng();
        for node in self.nodes.iter_mut() {
            if rng.random::<f64>() < p {
                node.state = true
            }
        }
    }
}

impl SynchronousRBN {
    pub fn new(n: u32, k: u32, p: f64) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(n as usize);
        // Create nodes with ids: 0..n.
        for i in 0..n {
            let node = Node {
                id: i as usize,
                input_ids: SynchronousRBN::generate_input_ids(k, n),
                truth_table: generate_truth_table(k as usize, p),
                state: false,
            };
            nodes.push(node);
        }
        SynchronousRBN { nodes }
    }

    fn generate_input_ids(k: u32, n: u32) -> Vec<usize> {
        // SequentialRBNs randomly generate exactly k input ids.
        let mut rng = rand::rng();
        let input_ids = rand::seq::index::sample(&mut rng, n as usize, k as usize);
        input_ids.into_vec()
    }
}

/// Generate a random truth table of n inputs with probability p a given input combination
/// will return true.
fn generate_truth_table(n: usize, p: f64) -> HashMap<Vec<u8>, bool> {
    let mut rng = rand::rng();
    let num_rows = 2_usize.pow(n as u32);

    let mut truth_table = HashMap::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut input_vector = Vec::with_capacity(n);
        for j in (0..n).rev() {
            input_vector.push(((i >> j) & 1) as u8);
        }

        let output = if rng.random::<f64>() < p { true } else { false };
        truth_table.insert(input_vector, output);
    }

    truth_table
}

#[cfg(test)]
#[path = "unit_tests/rbn.rs"]
mod tests;
