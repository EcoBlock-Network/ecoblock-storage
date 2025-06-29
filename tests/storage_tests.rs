use ecoblock_core::{SensorData, TangleBlock, Signature};
use ecoblock_storage::{Tangle, TangleError};

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
