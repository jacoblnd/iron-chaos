use super::*;

struct DeterministicProvider {
    random_bool_return: bool,
    random_distinct_return: Vec<usize>,
}

impl DeterministicProvider {
    fn new() -> Self {
        Self {
            random_bool_return: false,
            random_distinct_return: Vec::new(),
        }
    }
}

impl RandProvider for DeterministicProvider {
    fn random_bool(&self, _p: f64) -> bool {
        self.random_bool_return
    }
    fn random_distinct(&self, _k: usize, _n: usize) -> Vec<usize> {
        self.random_distinct_return.clone()
    }
}

// SynchronousRBN tests

// SynchronousRBN.setup() tests

#[test]
fn sync_rbn_one_node_setup() {
    // One node setup:
    // One node with one connection from itself.
    // Truth tables always evalute to false.
    let mut test_rng = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rng.random_bool_return = false;
    // Used for determining input_ids.
    test_rng.random_distinct_return = vec![0];

    let mut s_rbn = SynchronousRBN {
        n: 1,
        k: 1,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rng,
    };
    s_rbn.setup_nodes();

    let expected_nodes: Vec<Node> = vec![Node {
        id: 0,
        input_ids: vec![0],
        truth_table: HashMap::from([(vec![0], false), (vec![1], false)]),
        state: false,
    }];

    assert_eq!(s_rbn.nodes, expected_nodes);
}

#[test]
fn sync_rbn_two_node_setup() {
    // Two node setup:
    // Two nodes with Two connections each - one from itself and one from the othe node.
    // Truth tables always evalute to true.
    let mut test_rng = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rng.random_bool_return = true;
    // Used for determining input_ids.
    test_rng.random_distinct_return = vec![0, 1];

    let mut s_rbn = SynchronousRBN {
        n: 2,
        k: 2,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rng,
    };
    s_rbn.setup_nodes();

    let expected_nodes: Vec<Node> = vec![
        Node {
            id: 0,
            input_ids: vec![0, 1],
            truth_table: HashMap::from([
                (vec![0, 0], true),
                (vec![0, 1], true),
                (vec![1, 0], true),
                (vec![1, 1], true),
            ]),
            state: false,
        },
        Node {
            id: 1,
            input_ids: vec![0, 1],
            truth_table: HashMap::from([
                (vec![0, 0], true),
                (vec![0, 1], true),
                (vec![1, 0], true),
                (vec![1, 1], true),
            ]),
            state: false,
        },
    ];

    assert_eq!(s_rbn.nodes, expected_nodes);
}

#[test]
fn sync_rbn_three_node_setup() {
    // Three node setup:
    // Three nodes with 2 connections each: always from nodes [1, 2].
    // Truth table evaluates to true for all inputs.
    let mut test_rng = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rng.random_bool_return = true;
    // Used for determining input_ids.
    test_rng.random_distinct_return = vec![1, 2];

    let mut s_rbn = SynchronousRBN {
        n: 3,
        k: 2,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rng,
    };
    s_rbn.setup_nodes();

    let expected_nodes: Vec<Node> = vec![
        Node {
            id: 0,
            input_ids: vec![1, 2],
            truth_table: HashMap::from([
                (vec![0, 0], true),
                (vec![0, 1], true),
                (vec![1, 0], true),
                (vec![1, 1], true),
            ]),
            state: false,
        },
        Node {
            id: 1,
            input_ids: vec![1, 2],
            truth_table: HashMap::from([
                (vec![0, 0], true),
                (vec![0, 1], true),
                (vec![1, 0], true),
                (vec![1, 1], true),
            ]),
            state: false,
        },
        Node {
            id: 2,
            input_ids: vec![1, 2],
            truth_table: HashMap::from([
                (vec![0, 0], true),
                (vec![0, 1], true),
                (vec![1, 0], true),
                (vec![1, 1], true),
            ]),
            state: false,
        },
    ];

    assert_eq!(s_rbn.nodes, expected_nodes);
}

// SynchronousRBN.advance() tests
#[test]
fn single_node() {
    let test_rng = DeterministicProvider::new();
}
