// WARNING:
// These Poseidon parameters are placeholder/demo parameters used only
// for prototyping the arkworks pipeline and Merkle-tree structure.
// They should not be treated as production cryptographic parameters.

use ark_bls12_381::Fr;
use ark_sponge::{
    poseidon::{PoseidonConfig, PoseidonSponge},
    CryptographicSponge,
};

pub fn hash(left: Fr, right: Fr) -> Fr {
    let config = poseidon_config();

    let mut sponge = PoseidonSponge::new(&config);

    sponge.absorb(&left);
    sponge.absorb(&right);

    sponge.squeeze_field_elements(1)[0]
}

fn poseidon_config() -> PoseidonConfig<Fr> {
    let full_rounds = 8;
    let partial_rounds = 57;
    let alpha = 5;

    let mds = vec![
        vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)],
        vec![Fr::from(4u64), Fr::from(5u64), Fr::from(6u64)],
        vec![Fr::from(7u64), Fr::from(8u64), Fr::from(9u64)],
    ];

    let ark = vec![
        vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)];
        full_rounds + partial_rounds
    ];

    PoseidonConfig::new(
        full_rounds,
        partial_rounds,
        alpha,
        mds,
        ark,
        2,
        1,
    )
}
