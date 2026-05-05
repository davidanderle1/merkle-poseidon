# Merkle Poseidon Prototype

This is a small Rust project I built to explore Merkle trees and zero-knowledge-friendly hashing using the arkworks library.

## What it does

- Builds a Merkle tree over field elements (`ark_bls12_381::Fr`)
- Uses a Poseidon-based hash (instead of a simple placeholder)
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

## Next steps

- Replace toy sum check with a zero-knowledge proof
- Move from simple verification to a circuit-based approach (arkworks R1CS)
- Explore proving that total sum > threshold without revealing individual values


Note: the current implementation checks the sum explicitly. The next step is to replace this with a zero-knowledge proof so the verifier learns only that the condition holds, not the values.