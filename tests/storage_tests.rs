use ecoblock_core::{SensorData, TangleBlock, Signature};
use ecoblock_storage::{Tangle, TangleError};
use std::fs;
use blake3;

fn dummy_block(id: &str, parents: Vec<&str>) -> TangleBlock {
    TangleBlock {
        id: id.to_string(),
        parents: parents.into_iter().map(|s| s.to_string()).collect(),
        data: SensorData {
            pm25: 10.0,
            co2: 400.0,
            temperature: 20.0,
            humidity: 50.0,
            timestamp: 123456,
        },
        signature: Signature("dummy_sig".into()),
    }
}

#[test]
fn insert_genesis_block() {
    let mut tangle = Tangle::new();
    let block = dummy_block("A", vec![]);
    let result = tangle.insert(block);
    assert!(result.is_ok());
    assert_eq!(tangle.len(), 1);
}

#[test]
fn reject_block_with_missing_parent() {
    let mut tangle = Tangle::new();
    let block = dummy_block("B", vec!["Z"]);
    let result = tangle.insert(block);
    assert!(matches!(result, Err(TangleError::MissingParent(_))));
    assert_eq!(tangle.len(), 0);
}

#[test]
fn insert_block_with_valid_parent() {
    let mut tangle = Tangle::new();
    let genesis = dummy_block("A", vec![]);
    tangle.insert(genesis).unwrap();
    let child = dummy_block("B", vec!["A"]);
    let result = tangle.insert(child);
    assert!(result.is_ok());
    assert_eq!(tangle.len(), 2);
    assert!(tangle.get("B").is_some());
}



#[test]
fn save_and_load_tangle_from_file() {
    let path = "test_tangle.json";

    let mut tangle = Tangle::new();
    let block_a = dummy_block("A", vec![]);
    let block_b = dummy_block("B", vec!["A"]);

    tangle.insert(block_a).unwrap();
    tangle.insert(block_b).unwrap();

    tangle.save_to_file(path).unwrap();
    let loaded = Tangle::load_from_file(path).unwrap();

    assert_eq!(loaded.len(), 2);
    assert!(loaded.get("B").is_some());

    let _ = fs::remove_file(path);
}

#[test]
fn create_unsigned_block_and_check_id_is_hash() {

    let data = SensorData {
        pm25: 12.0,
        co2: 420.0,
        temperature: 24.0,
        humidity: 38.0,
        timestamp: 123456789,
    };

    let parents = vec!["A".to_string(), "B".to_string()];
    let block = TangleBlock::new_unsigned(parents.clone(), data.clone());

    let payload = serde_json::to_vec(&(parents, data)).unwrap();
    let expected_id = blake3::hash(&payload).to_hex().to_string();
    assert_eq!(block.id, expected_id);

}

#[test]
fn print_tangle_structure_to_terminal() {
    let mut tangle = Tangle::new();
    let block_a = TangleBlock::new_unsigned(vec![], SensorData {
        pm25: 10.0, co2: 400.0, temperature: 22.0, humidity: 55.0, timestamp: 123,
    });
    let block_b = TangleBlock::new_unsigned(vec![block_a.id.clone()], SensorData {
        pm25: 11.0, co2: 410.0, temperature: 23.0, humidity: 56.0, timestamp: 124,
    });
    let block_c = TangleBlock::new_unsigned(vec![block_a.id.clone(), block_b.id.clone()], SensorData {
        pm25: 12.0, co2: 420.0, temperature: 24.0, humidity: 57.0, timestamp: 125,
    });

    println!("Block A: {}", block_a.id);
    println!("Block B: {}", block_b.id);
    println!("Block C: {}", block_c.id);

    tangle.insert(block_a).unwrap();
    tangle.insert(block_b).unwrap();
    tangle.insert(block_c).unwrap();

    tangle.pretty_print();
}
