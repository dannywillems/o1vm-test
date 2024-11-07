/// Generic trait to represent a field
pub trait Field:
    Copy
    + Clone
    + core::ops::Add<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Neg<Output = Self>
{
    const ZERO: Self;

    const ONE: Self;

    /// The number of machine words needed to represent the field element.
    /// As we compile for 32bit architectures, the word size is 32 bits.
    const WORDS: usize;

    /// The modulus of the field.
    const SIZE: usize;

    fn new(value: u32) -> Self;
}

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

#[derive(Copy, Clone)]
pub struct BabyBear {
    pub value: u32,
}

impl Field for BabyBear {
    const ZERO: BabyBear = BabyBear { value: 0 };

    const ONE: BabyBear = BabyBear { value: 1 };

    const WORDS: usize = 1;

    const SIZE: usize = 15 * (1 << 27) + 1;

    fn new(value: u32) -> Self {
        BabyBear {
            value: value % Self::SIZE as u32,
        }
    }
}

impl core::ops::Add for BabyBear {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        BabyBear {
            value: (self.value + other.value) % Self::SIZE as u32,
        }
    }
}

impl core::ops::Mul for BabyBear {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        BabyBear {
            value: (self.value * other.value) % Self::SIZE as u32,
        }
    }
}

impl core::ops::Neg for BabyBear {
    type Output = Self;

    fn neg(self) -> Self {
        BabyBear {
            value: (Self::SIZE as u32 - self.value) % Self::SIZE as u32,
        }
    }
}
