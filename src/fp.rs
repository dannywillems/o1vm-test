use crate::bigint::BigInt32;

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
}

/// Represents a field element
pub trait Field32: Field {
    fn new(value: u32) -> Self;
}

pub type BabyBear = BigInt32;

impl Field for BabyBear {
    const ZERO: BabyBear = BigInt32 { data: [0] };

    const ONE: BabyBear = BigInt32 { data: [1] };

    const WORDS: usize = 1;

    const SIZE: usize = 15 * (1 << 27) + 1;
}

impl Field32 for BabyBear {
    fn new(value: u32) -> Self {
        Self {
            data: [value % Self::SIZE as u32],
        }
    }
}

impl core::ops::Add for BabyBear {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: [(self.data[0] + other.data[0]) % Self::SIZE as u32],
        }
    }
}

impl core::ops::Mul for BabyBear {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: [(self.data[0] * other.data[0]) % Self::SIZE as u32],
        }
    }
}

impl core::ops::Neg for BabyBear {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: [Self::SIZE as u32 - self.data[0]],
        }
    }
}
