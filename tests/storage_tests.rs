use ecoblock_core::{SensorData, TangleBlockData};
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_storage::{tangle::block::TangleBlock, Tangle, TangleError};

use std::time::{SystemTime, UNIX_EPOCH};

fn dummy_block(parents: Vec<String>) -> TangleBlock {
    let keypair = CryptoKeypair::generate(); // ✅ ajout
    let data = SensorData {
        pm25: 10.0,
        co2: 400.0,
        temperature: 20.0,
        humidity: 50.0,
        timestamp: 123456,
    };
    let tangle_data = TangleBlockData { parents, data };
    TangleBlock::new(tangle_data, &keypair)
}


#[test]
fn insert_genesis_block() {
    println!("=== insert_genesis_block ===");
    let mut tangle = Tangle::new();
    let block = dummy_block(vec![]);
    let result = tangle.insert(block.clone());

    println!("Inserted block ID: {}", block.id);
    assert!(result.is_ok());
    assert_eq!(tangle.len(), 1);
}

#[test]
fn reject_block_with_missing_parent() {
    println!("=== reject_block_with_missing_parent ===");
    let mut tangle = Tangle::new();

    let block = dummy_block(vec!["nonexistent_parent".to_string()]);
    let result = tangle.insert(block);

    println!("Résultat de l'insertion : {:?}", result);
    assert!(matches!(result, Err(TangleError::MissingParent(_))));
    assert_eq!(tangle.len(), 0);
}


#[test]
fn insert_block_with_valid_parent() {
    println!("=== insert_block_with_valid_parent ===");

    let mut tangle = Tangle::new();

    // Création et insertion du bloc genesis
    let genesis = dummy_block(vec![]);
    let parent_id = genesis.id.clone();
    tangle.insert(genesis).unwrap();

    // Création du bloc enfant avec le parent valide
    let child = dummy_block(vec![parent_id.clone()]);
    let result = tangle.insert(child.clone());

    println!("Bloc enfant inséré avec le parent : {}", parent_id);
    println!("Résultat : {:?}", result);

    assert!(result.is_ok());
    assert_eq!(tangle.len(), 2);
    assert!(tangle.get(&child.id).is_some());
}

#[test]
fn test_insert_block() {
    println!("=== test_insert_block ===");

    // Génère une paire de clés
    let keypair = CryptoKeypair::generate();

    // Prépare les données
    let data = TangleBlockData {
        parents: vec![], // bloc genesis
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

    // Crée un bloc signé
    let block = TangleBlock::new(data, &keypair);
    println!("ID du bloc généré : {}", block.id);
    println!("Clé publique : {}", block.public_key);
    println!("Signature : {}", block.signature.0);

    // Insert dans le Tangle
    let mut tangle = Tangle::new();
    let result = tangle.insert(block.clone());

    println!("Résultat : {:?}", result);
    assert!(result.is_ok());
    assert_eq!(tangle.len(), 1);
    assert!(tangle.get(&block.id).is_some());
}

