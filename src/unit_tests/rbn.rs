use super::*;

struct DeterministicRBN {}

impl DeterministicRBN {
    fn new() -> Self {
        Self {}
    }
}

impl RandProvider for DeterministicRBN {
    fn random_bool(&self, p: f64) -> bool {
        true
    }
    fn random_distinct(&self, k: usize, n: usize) -> Vec<usize> {
        vec![0; n]
    }
}

#[test]
fn single_node() {
    let test_rng = DeterministicRBN::new();

    // Create a single-node rbn.
    let mut s_rbn = SynchronousRBN {
        n: 1,
        k: 1,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rng,
    };
    s_rbn.setup_nodes();

    assert_eq!(s_rbn.nodes[0].state, false);
}
