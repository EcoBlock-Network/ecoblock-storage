use base64ct::Encoding;
use ecoblock_core::{SensorData, TangleBlockData};
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_storage::{tangle::block::TangleBlock, Tangle, TangleError};
use std::time::{SystemTime, UNIX_EPOCH};

/// Crée un bloc Genesis valide
fn create_genesis_block(keypair: &CryptoKeypair) -> TangleBlock {
    let sensor_data = SensorData {
        pm25: 0.0,
        co2: 0.0,
        temperature: 0.0,
        humidity: 0.0,
        timestamp: 0,
    };
    let block_data = TangleBlockData {
        parents: vec![],
        data: sensor_data,
    };
    TangleBlock::new(block_data, keypair)
}

fn dummy_block_with_keypair(parents: Vec<String>, keypair: &CryptoKeypair) -> TangleBlock {
    let data = SensorData {
        pm25: 10.0,
        co2: 400.0,
        temperature: 20.0,
        humidity: 50.0,
        timestamp: 123456,
    };
    let tangle_data = TangleBlockData { parents, data };
    TangleBlock::new(tangle_data, keypair)
}

#[test]
fn insert_genesis_block() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();
    let block = create_genesis_block(&keypair);

    println!("[GENESIS] ID: {}", block.id);

    let result = tangle.insert(block.clone());
    assert!(result.is_ok(), "Failed to insert genesis block");
    assert_eq!(tangle.len(), 1, "Tangle should contain exactly one block");
}

#[test]
fn reject_block_with_missing_parent() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();
    let block = dummy_block_with_keypair(vec!["nonexistent_parent".to_string()], &keypair);

    let result = tangle.insert(block);
    assert!(
        matches!(result, Err(TangleError::MissingParent(_))),
        "Block with missing parent should be rejected"
    );
    assert_eq!(tangle.len(), 0, "Tangle should remain empty");
}

#[test]
fn insert_block_with_valid_parent() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();

    let genesis = create_genesis_block(&keypair);
    let parent_id = genesis.id.clone();
    tangle.insert(genesis.clone()).unwrap();

    let child = dummy_block_with_keypair(vec![parent_id.clone()], &keypair);
    let result = tangle.insert(child.clone());

    println!("[PARENT] ID: {}", parent_id);
    println!("[CHILD ] ID: {}", child.id);

    assert!(result.is_ok(), "Failed to insert block with valid parent");
    assert_eq!(tangle.len(), 2, "Tangle should contain two blocks");
    assert!(tangle.get(&child.id).is_some(), "Child block should exist in the tangle");
}

#[test]
fn test_insert_block() {
    let keypair = CryptoKeypair::generate();

    let data = TangleBlockData {
        parents: vec![],
        data: SensorData {
            pm25: 12.5,
            co2: 400.0,
            temperature: 22.5,
            humidity: 45.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    };

    let block = TangleBlock::new(data, &keypair);
    let mut tangle = Tangle::new();
    let result = tangle.insert(block.clone());

    assert!(result.is_ok(), "Failed to insert block");
    assert_eq!(tangle.len(), 1, "Tangle should contain exactly one block");
    assert!(tangle.get(&block.id).is_some(), "Block should exist in the tangle");
}

#[test]
fn test_insert_valid_block() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();

    let block = create_genesis_block(&keypair);
    let result = tangle.insert(block.clone());

    assert!(result.is_ok(), "Failed to insert valid block");
    assert_eq!(tangle.len(), 1, "Tangle should contain exactly one block");
}

#[test]
fn test_insert_block_missing_parent_should_fail() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();

    let data = TangleBlockData {
        parents: vec!["nonexistent_parent".to_string()],
        data: SensorData {
            pm25: 0.0,
            co2: 0.0,
            temperature: 0.0,
            humidity: 0.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    };

    let block = TangleBlock::new(data, &keypair);
    let result = tangle.insert(block);

    assert!(
        result.is_err(),
        "Insertion should fail for block with missing parent"
    );
}

#[test]
fn test_insert_block_with_invalid_signature_should_fail() {
    let mut tangle = Tangle::new();
    let keypair = CryptoKeypair::generate();
    let wrong_keypair = CryptoKeypair::generate();

    let data = TangleBlockData {
        parents: vec![],
        data: SensorData {
            pm25: 15.0,
            co2: 420.0,
            temperature: 25.0,
            humidity: 55.0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    };

    let mut block = TangleBlock::new(data, &wrong_keypair);
    block.public_key = base64ct::Base64::encode_string(keypair.public_key().as_bytes());

    println!("Attempting to insert block with ID: {}", block.id);
    println!("Block parents: {:?}", block.data.parents);
    println!("Block public key: {}", block.public_key);
    println!("Block signature: {:?}", block.signature);

    let result = tangle.insert(block);

    if let Err(err) = &result {
        println!("Insertion failed with error: {:?}", err);
    }

    assert!(
        result.is_err(),
        "Insertion should fail for block with invalid signature"
    );
}
