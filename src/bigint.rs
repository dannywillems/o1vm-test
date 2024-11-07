#[derive(Clone, Copy)]
pub struct BigInt<const N: usize> {
    pub data: [u32; N],
}

impl<const N: usize> Default for BigInt<N> {
    fn default() -> Self {
        BigInt { data: [0; N] }
    }
}

pub type BigInt32 = BigInt<1>;
pub type BigInt64 = BigInt<2>;
pub type BigInt128 = BigInt<4>;
pub type BigInt256 = BigInt<8>;
