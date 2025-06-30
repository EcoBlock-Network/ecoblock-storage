use ecoblock_core::domain::tangle_data::TangleBlockData;
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_core::{Signature, Signable};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
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
        let payload = data.payload();
        let id = blake3::hash(&payload).to_hex().to_string();

        let signature_raw = keypair.sign(&payload);
        let signature = Signature(Base64::encode_string(signature_raw.to_bytes().as_ref()));
        let public_key = Base64::encode_string(keypair.public_key().as_bytes());

        Self {
            id,
            parents: data.parents.clone(),
            data,
            signature,
            public_key,
        }
    }
}
