use ark_bls12_381::Fr;

pub fn hash(left: Fr, right: Fr) -> Fr {
    left + right
}