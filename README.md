# ecoblock-storage

Local data persistence and Tangle implementation for the Ecoblock mesh network.

## 📦 Purpose

The `ecoblock-storage` crate is responsible for:

- Managing a local **DAG-based Tangle** structure.
- Storing and indexing **signed sensor data blocks**.
- Validating block signatures and parent references.
- Providing **fast insertion, retrieval**, and **graph traversal**.
- Persisting the entire Tangle as a human-readable JSON file.

---

## 📐 Architecture

This crate follows clean architecture principles and is structured into:

- `block.rs`: Block structure containing sensor data, parent links, ID, signature, and public key.
- `graph.rs`: Lightweight DAG representation (nodes and edges).
- `storage.rs`: Main `Tangle` structure that handles insertion, validation, and persistence.
- `tests/`: Comprehensive integration tests for all logic (signature verification, parent resolution, etc.).

---

## ✅ Features

### TangleBlock
Each block is:
- Signed using Ed25519 (via [`ecoblock-crypto`](../ecoblock-crypto))
- Identified by a hash of its serialized payload
- Linked to zero or more parent blocks (for causality)

### Validation
Before insertion, a block must:
- Have all parent blocks present in the Tangle
- Have a valid Ed25519 signature on the payload
- Be uniquely identified by a stable hash (block ID)

### Persistence
The Tangle can be:
- Saved to disk as a JSON array of blocks
- Reloaded from disk and validated
- Printed as a simple DAG for debugging

---

## 🧪 Testing

Run the test suite:

```bash
cargo test -- --nocapture
````

Covered scenarios include:

* Genesis block insertion
* Valid/invalid signature rejection
* Missing parent handling
* DAG integrity and block ordering

---

## 🔧 Example

```rust
let mut tangle = Tangle::new();
let genesis = TangleBlock::new(sensor_data, &keypair)?;
tangle.insert(genesis)?;
tangle.save_to_file("tangle.json")?;
```

---

## 📁 File format

```json
[
  {
    "id": "...",
    "parents": [],
    "data": { "pm25": ..., "timestamp": ... },
    "signature": "...",
    "public_key": "..."
  },
  ...
]
```

---

## 📚 Related Crates

* [`ecoblock-core`](../ecoblock-core): Core domain models
* [`ecoblock-crypto`](../ecoblock-crypto): Cryptographic primitives and signature verification

---

## 🔒 Security

Blocks must be verified using:

* BLAKE3 for deterministic payload hashing
* Ed25519 for signatures

All data is immutable once inserted.

---

## 📌 Status

✅ MVP complete
🧪 Test coverage: **high**
📁 Disk persistence: implemented
📈 DAG traversal: implemented
🔗 Network sync: pending (handled in `ecoblock-mesh`)

---

## License

MIT or Apache 2.0


