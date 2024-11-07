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
