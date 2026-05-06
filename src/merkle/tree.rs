use crate::hash::poseidon::hash;
use super::proof::MerkleProof;
use ark_bls12_381::Fr;

#[derive(Debug)]
pub struct MerkleTree {
    pub leaves: Vec<Fr>,
    pub layers: Vec<Vec<Fr>>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Fr>) -> Self {
        assert!(!leaves.is_empty());

        let mut layers = Vec::new();
        layers.push(leaves.clone());

        let mut current = leaves;

        while current.len() > 1 {
            let mut next = Vec::new();

            for pair in current.chunks(2) {
                let left = pair[0];
                let right = if pair.len() == 2 { pair[1] } else { pair[0] };

                
                let parent = hash(left, right);
                next.push(parent);
            }

            layers.push(next.clone());
            current = next;
        }

        MerkleTree {
            leaves: layers[0].clone(),
            layers,
        }
    }

    pub fn root(&self) -> Fr {
        self.layers.last().unwrap()[0]
    }

    pub fn generate_proof(&self, mut index: usize) -> MerkleProof {
        let mut siblings = Vec::new();

        for layer in &self.layers {
            if layer.len() == 1 {
                break;
            }

            let is_right = index % 2 == 1;

            let sibling_index = if is_right {
                index - 1
            } else {
                if index + 1 < layer.len() {
                    index + 1
                } else {
                    index
                }
            };

            siblings.push(layer[sibling_index]);

            index /= 2;
        }

        MerkleProof {
            siblings,
        }
    }

    pub fn verify_proof(
        root: Fr,
        leaf: Fr,
        mut index: usize,
        proof: &MerkleProof,
    ) -> bool {
        let mut hash_val = leaf;

        for sibling in &proof.siblings {
            if index % 2 == 0 {
                hash_val = hash(hash_val, *sibling);
            } else {
                hash_val = hash(*sibling, hash_val);
            }

            index /= 2;
        }

        hash_val == root
    }
}
