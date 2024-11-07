use crate::fp::Fp;
use core::unimplemented;

// pub trait Poseidon {
//     type Field: Field;

//     /// The s-box used in the Poseidon permutation.
//     const ALPHA: usize;

//     fn apply_permutation<
//         const STATE_SIZE: usize,
//         const NB_FULL_ROUNDS: usize,
//         const NB_PARTIAL_ROUNDS: usize,
//         const NB_TOTAL_ROUNDS: usize,
//     >(
//         inputs: [Self::Field; STATE_SIZE],
//         round_constants: [[Self::Field; STATE_SIZE]; NB_TOTAL_ROUNDS],
//     ) -> [Self::Field; STATE_SIZE] {
//         assert_eq!(NB_FULL_ROUNDS + NB_PARTIAL_ROUNDS, NB_TOTAL_ROUNDS);
//         let mut state = inputs;
//         let state = (0..NB_FULL_ROUNDS / 2).map(|round| {
//             let state = state
//                 .into_iter()
//                 .enumerate()
//                 .map(|(j, s)| s + round_constants[round][j]);
//             state
//         });
//         inputs
//     }
// }

pub fn poseidon_hash(_inputs: [Fp; 3]) -> [Fp; 3] {
    unimplemented!("TODO")
}
