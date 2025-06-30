use std::fs;
use std::path::Path;
use serde_json;

use crate::tangle::block::TangleBlock;

use super::graph::Graph;

pub struct Tangle {
    graph: Graph,
}

#[derive(Debug)]
pub enum TangleError {
    MissingParent(String),
}

impl Tangle {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    pub fn insert(&mut self, block: TangleBlock) -> Result<(), TangleError> {
        for parent_id in &block.parents {
            if !self.graph.contains(parent_id) {
                return Err(TangleError::MissingParent(parent_id.clone()));
            }
        }
        self.graph.insert(block);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&TangleBlock> {
        self.graph.get(id)
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    /// Persist the Tangle to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let all_blocks: Vec<&TangleBlock> = self.graph.blocks.values().collect();
        let json = serde_json::to_string_pretty(&all_blocks)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load a Tangle from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let blocks: Vec<TangleBlock> = serde_json::from_str(&content)?;
        let mut tangle = Tangle::new();
        for block in blocks {
            let _ = tangle.insert(block);
        }
        Ok(tangle)
    }

    /// Print a simplified view of the DAG
    pub fn pretty_print(&self) {
        for block in self.graph.blocks.values() {
            if block.parents.is_empty() {
                println!("Block {} (genesis)", block.id);
            } else {
                println!("Block {} → {:?}", block.id, block.parents);
            }
        }
    }
}
