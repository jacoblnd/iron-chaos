use super::*;

#[test]
fn has_size() {
    let rbn = RBN { size: 20 };
    assert!(rbn.size == 20);
}
