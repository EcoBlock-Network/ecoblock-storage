use std::collections::HashMap;
use std::fs;
use std::path::Path;
use ecoblock_core::Signable;
use serde_json;
use crate::tangle::block::TangleBlock;
use ecoblock_crypto::signature::Signature;
use hex;

use super::graph::Graph;

pub struct Tangle {
    graph: Graph,
    blocks: HashMap<String, TangleBlock>, 
}

#[derive(Debug)]
pub enum TangleError {
    MissingParent(String),
    InvalidSignature,
}

impl Tangle {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: TangleBlock) -> Result<(), TangleError> {
        for parent_id in &block.parents {
            if !self.blocks.contains_key(parent_id) {
                return Err(TangleError::MissingParent(parent_id.clone()));
            }
        }
    
        let payload = block.data.payload();
    
        let is_valid = block.signature.verify(&block.public_key, &payload);
    
        if !is_valid {
            return Err(TangleError::InvalidSignature);
        }
    
        self.blocks.insert(block.id.clone(), block.clone());
        self.graph.add_node(block.id.clone());
    
        for parent_id in &block.parents {
            self.graph.add_edge(parent_id.clone(), block.id.clone());
        }
        
        Ok(())
    }
    

    pub fn get(&self, id: &str) -> Option<&TangleBlock> {
        self.blocks.get(id)
    }    

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
    

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let all_blocks: Vec<&TangleBlock> = self.blocks.values().collect();
        let json = serde_json::to_string_pretty(&all_blocks)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let blocks: Vec<TangleBlock> = serde_json::from_str(&content)?;
        let mut tangle = Tangle::new();
        for block in blocks {
            let _ = tangle.insert(block);
        }
        Ok(tangle)
    }

    pub fn pretty_print(&self) {
        for block in self.blocks.values() {
            if block.parents.is_empty() {
                println!("Block {} (genesis)", block.id);
            } else {
                println!("Block {} → {:?}", block.id, block.parents);
            }
        }
    }
}
