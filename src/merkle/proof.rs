use ark_bls12_381::Fr;

#[derive(Debug)]
pub struct MerkleProof {
    pub siblings: Vec<Fr>,
}
