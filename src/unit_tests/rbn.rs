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
    let mut test_rand_provider = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rand_provider.random_bool_return = false;
    // Used for determining input_ids.
    test_rand_provider.random_distinct_return = vec![0];

    let mut sync_rbn = SynchronousRBN {
        n: 1,
        k: 1,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rand_provider,
    };
    sync_rbn.setup_nodes();

    let expected_nodes: Vec<Node> = vec![Node {
        id: 0,
        input_ids: vec![0],
        truth_table: HashMap::from([(vec![0], false), (vec![1], false)]),
        state: false,
    }];

    assert_eq!(sync_rbn.nodes, expected_nodes);
}

#[test]
fn sync_rbn_two_node_setup() {
    // Two node setup:
    // Two nodes with Two connections each - one from itself and one from the othe node.
    // Truth tables always evalute to true.
    let mut test_rand_provider = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rand_provider.random_bool_return = true;
    // Used for determining input_ids.
    test_rand_provider.random_distinct_return = vec![0, 1];

    let mut sync_rbn = SynchronousRBN {
        n: 2,
        k: 2,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rand_provider,
    };
    sync_rbn.setup_nodes();

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

    assert_eq!(sync_rbn.nodes, expected_nodes);
}

#[test]
fn sync_rbn_three_node_setup() {
    // Three node setup:
    // Three nodes with 2 connections each: always from nodes [1, 2].
    // Truth table evaluates to true for all inputs.
    let mut test_rand_provider = DeterministicProvider::new();
    // Used for determining if truth tables evaluate to true.
    test_rand_provider.random_bool_return = true;
    // Used for determining input_ids.
    test_rand_provider.random_distinct_return = vec![1, 2];

    let mut sync_rbn = SynchronousRBN {
        n: 3,
        k: 2,
        p: 1 as f64,
        nodes: Vec::new(),
        random_provider: test_rand_provider,
    };
    sync_rbn.setup_nodes();

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

    assert_eq!(sync_rbn.nodes, expected_nodes);
}

// SynchronousRBN.advance() tests
#[test]
fn sync_rbn_single_node_advance() {
    // Single node which activates itself when it is true in previous time step.
    let test_rand_provider = DeterministicProvider::new();
    let mut sync_rbn = SynchronousRBN {
        n: 1,
        k: 1,
        p: 1 as f64,
        nodes: vec![Node {
            id: 0,
            input_ids: vec![0], // input is itself
            truth_table: HashMap::from([
                (vec![0], false),
                (vec![1], true), // only activates when it is true.
            ]),
            state: false,
        }],
        random_provider: test_rand_provider,
    };

    // Since it is initialized to false, should not activate itself.
    assert_eq!(sync_rbn.advance(1), vec![0]);
    // Set state to 1 and check next state;
    sync_rbn.nodes[0].state = true;
    assert_eq!(sync_rbn.advance(1), vec![1]);
}

#[test]
fn sync_rbn_two_node_advance() {
    // Two nodes: each depend on the other and activate if prevoius time step was true.
    let test_rand_provider = DeterministicProvider::new();
    let mut sync_rbn = SynchronousRBN {
        n: 2,
        k: 1,
        p: 1 as f64,
        nodes: vec![
            Node {
                id: 0,
                input_ids: vec![1], // input is other node: id: 1
                truth_table: HashMap::from([
                    (vec![0], false),
                    (vec![1], true), // only activates when input is true.
                ]),
                state: false,
            },
            Node {
                id: 1,
                input_ids: vec![0], // input is other node: id: 2
                truth_table: HashMap::from([
                    (vec![0], false),
                    (vec![1], true), // only activates when input is true.
                ]),
                state: false,
            },
        ],
        random_provider: test_rand_provider,
    };

    // Both are initialized to false, should not activate.
    assert_eq!(sync_rbn.advance(1), vec![0, 0]);
    // Set state of node: 0 to true, next time step should have node 1 being true
    // while node: 0 is false.
    sync_rbn.nodes[0].state = true;
    assert_eq!(sync_rbn.advance(1), vec![0, 1]);
    // Should ping pong back and forth.
    assert_eq!(sync_rbn.advance(1), vec![1, 0]);
    assert_eq!(sync_rbn.advance(1), vec![0, 1]);
}
