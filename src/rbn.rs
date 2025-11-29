#[derive(Debug)]
pub struct RBN {
    pub size: u32,
}

#[cfg(test)]
#[path = "unit_tests/rbn.rs"]
mod tests;
