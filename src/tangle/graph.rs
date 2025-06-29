use std::collections::HashMap;
use ecoblock_core::TangleBlock;

/// In-memory DAG to store Tangle blocks
pub struct Graph {
    blocks: HashMap<String, TangleBlock>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: TangleBlock) {
        self.blocks.insert(block.id.clone(), block);
    }

    pub fn get(&self, id: &str) -> Option<&TangleBlock> {
        self.blocks.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.blocks.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}
