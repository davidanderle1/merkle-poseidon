use ark_bls12_381::Fr;

#[derive(Debug)]
pub struct MerkleProof {
    pub index: usize,
    pub siblings: Vec<Fr>,
}