use std::collections::{HashMap, HashSet};

pub struct Graph {
    pub(crate) blocks: HashMap<String, crate::tangle::block::TangleBlock>,
    pub(crate) edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: crate::tangle::block::TangleBlock) {
        let block_id = block.id.clone();
        self.blocks.insert(block_id.clone(), block);
        self.edges.entry(block_id).or_default();
    }

    pub fn add_node(&mut self, id: String) {
        self.edges.entry(id).or_insert_with(HashSet::new);
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn get(&self, id: &str) -> Option<&crate::tangle::block::TangleBlock> {
        self.blocks.get(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.blocks.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }    

    pub fn children(&self, id: &str) -> Option<&HashSet<String>> {
        self.edges.get(id)
    }
}