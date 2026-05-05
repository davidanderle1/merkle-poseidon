mod merkle;
mod hash;

use ark_bls12_381::Fr;
use merkle::tree::MerkleTree;
use crate::hash::poseidon::hash;

fn main() {
    let leaves = vec![
        (Fr::from(10u64), Fr::from(111u64)),
        (Fr::from(20u64), Fr::from(222u64)),
        (Fr::from(30u64), Fr::from(333u64)),
        (Fr::from(40u64), Fr::from(444u64)),
    ];

    let commitments: Vec<Fr> = leaves
        .iter()
        .map(|(value, rand)| hash(*value, *rand))
        .collect();

    let tree = MerkleTree::new(commitments.clone());

    let root = tree.root();
    println!("Root: {:?}", root);

    let index = 2;
    let proof = tree.generate_proof(index);

    let valid = MerkleTree::verify_proof(
        root,
        commitments[index],
        index,
        &proof,
    );

    println!("Proof valid: {}", valid);

    let sum: Fr = leaves
        .iter()
        .map(|(v, _)| *v)
        .fold(Fr::from(0u64), |acc, x| acc + x);

    println!("Actual sum: {:?}", sum);

    let threshold = Fr::from(50u64);

    println!("Sum > threshold: {}", sum > threshold);
}