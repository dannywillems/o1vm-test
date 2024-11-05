use core::unimplemented;

/// Represents a field element in a field of maximum size 2^256
pub struct Fp {
    #[allow(dead_code)]
    value: [u32; 8],
}

impl Fp {
    #[allow(dead_code)]
    fn new(value: [u32; 8]) -> Self {
        Fp { value }
    }
}

impl core::ops::Add for Fp {
    type Output = Self;

    fn add(self, _other: Self) -> Self {
        unimplemented!("TODO")
    }
}

impl core::ops::Sub for Fp {
    type Output = Self;

    fn sub(self, _other: Self) -> Self {
        unimplemented!("TODO")
    }
}

impl core::ops::Mul for Fp {
    type Output = Self;

    fn mul(self, _other: Self) -> Self {
        unimplemented!("TODO")
    }
}

impl core::ops::Div for Fp {
    type Output = Self;

    fn div(self, _other: Self) -> Self {
        unimplemented!("TODO")
    }
}
