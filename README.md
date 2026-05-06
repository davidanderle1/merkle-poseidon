# Merkle Poseidon Prototype

This is a Rust/arkworks prototype for exploring Merkle trees, zk-friendly hashing, and privacy-preserving balance commitments.

## What it does

- Builds a Merkle tree over field elements (`ark_bls12_381::Fr`)
- Uses a Poseidon-based hash interface
- Stores commitments in leaves:

  `commitment = hash(value, randomness)`

- Computes the Merkle root
- Generates and verifies Merkle proofs
- Computes a simple sum over the hidden values and compares it to a threshold

## Example flow

- Start with values like: 10, 20, 30, 40
- Add randomness to each
- Hash them into commitments
- Build a Merkle tree from those commitments
- Verify membership of a leaf
- Check if total sum > threshold

## Run

```bash
cargo run
```

## Current limitations

- The current threshold check is not zero knowledge and is computed natively in Rust.
- The current Poseidon parameters are manual placeholder/demo parameters used for prototyping and should not be treated as production cryptographic parameters.
- Balance values are currently represented as field elements, not bounded integer amounts with range constraints.
- This repository is an experimental research prototype, not production cryptography.

## Next steps

- Replace toy sum check with a zero-knowledge proof
- Move from simple verification to a circuit-based approach (arkworks R1CS)
- Explore proving that total sum > threshold without revealing individual values
