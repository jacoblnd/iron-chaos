use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: usize,
    input_ids: Vec<usize>,
    truth_table: HashMap<Vec<u8>, u8>,
    state: bool,
}

#[derive(Debug)]
pub struct SequentialRBN {
    nodes: Vec<Node>,
}

pub trait RBN {
    fn advance(t: u32) -> Vec<u8>;
    fn rand_activate(&mut self, p: f64);
}

impl RBN for SequentialRBN {
    fn advance(t: u32) -> Vec<u8> {
        todo!()
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

impl SequentialRBN {
    pub fn new(n: u32, k: u32, p: f64) -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(n as usize);
        // Create nodes with ids: 0..n.
        for i in 0..n {
            let node = Node {
                id: i as usize,
                input_ids: SequentialRBN::generate_input_ids(k, n),
                truth_table: generate_truth_table(k as usize, p),
                state: false,
            };
            nodes.push(node);
        }
        SequentialRBN { nodes }
    }

    fn generate_input_ids(k: u32, n: u32) -> Vec<usize> {
        // SequentialRBNs randomly generate exactly k input ids.
        let mut rng = rand::rng();
        let input_ids = rand::seq::index::sample(&mut rng, n as usize, k as usize);
        input_ids.into_vec()
    }
}

/*
Generate a random truth table of n inputs with probability p a given input combination
will return true.
*/
fn generate_truth_table(n: usize, p: f64) -> HashMap<Vec<u8>, u8> {
    let mut rng = rand::rng();
    let num_rows = 2_usize.pow(n as u32);

    let mut truth_table = HashMap::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut input_vector = Vec::with_capacity(n);
        for j in (0..n).rev() {
            input_vector.push(((i >> j) & 1) as u8);
        }

        let output = if rng.random::<f64>() < p { 1 } else { 0 };
        truth_table.insert(input_vector, output);
    }

    truth_table
}

#[cfg(test)]
#[path = "unit_tests/rbn.rs"]
mod tests;
