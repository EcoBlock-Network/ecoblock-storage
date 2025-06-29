use ecoblock_core::TangleBlock;

use super::graph::Graph;

/// Represents a local in-memory Tangle DAG
pub struct Tangle {
    graph: Graph,
}

/// Possible errors when working with the Tangle
#[derive(Debug)]
pub enum TangleError {
    MissingParent(String),
}

impl Tangle {
    /// Create a new empty Tangle
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    /// Insert a new block, validating that all parents exist
    pub fn insert(&mut self, block: TangleBlock) -> Result<(), TangleError> {
        for parent_id in &block.parents {
            if !self.graph.contains(parent_id) {
                return Err(TangleError::MissingParent(parent_id.clone()));
            }
        }
        self.graph.insert(block);
        Ok(())
    }

    /// Retrieve a block by its ID
    pub fn get(&self, id: &str) -> Option<&TangleBlock> {
        self.graph.get(id)
    }

    /// Total number of blocks in the Tangle
    pub fn len(&self) -> usize {
        self.graph.len()
    }
}
