use ecoblock_core::domain::tangle_data::TangleBlockData;
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_crypto::signature::Signature;
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use ecoblock_core::Signable;
use blake3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleBlock {
    pub id: String,
    pub parents: Vec<String>,
    pub data: TangleBlockData,
    pub signature: Signature,
    pub public_key: String,
}

impl TangleBlock {
    pub fn new(data: TangleBlockData, keypair: &CryptoKeypair) -> Self {
        let parents = data.parents.clone();
    
        let payload = data.payload();
        let id = blake3::hash(&payload).to_hex().to_string();
    
        let signature = keypair.sign(&payload);
        let public_key = Base64::encode_string(&keypair.public_key().to_bytes());
    
        Self {
            id,
            parents,
            data,
            signature,
            public_key,
        }
    }
    
}
