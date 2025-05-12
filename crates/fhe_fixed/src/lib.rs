//! FHE Fixed-point Arithmetic API

use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};
use tfhe_rs::integer::Ciphertext;
use tfhe_rs::prelude::{ClientKey, ServerKey};
use tfhe_rs::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_KS_PBS;

/// Lazily initialize a global ClientKey (and discard the ServerKey)
static CLIENT_KEY: Lazy<ClientKey> = Lazy::new(|| {
    let (ck, _sk) = tfhe_rs::integer::gen_keys_radix(PARAM_MESSAGE_2_CARRY_2_KS_PBS, 4);
    ck
});

/// Returns a reference to the global ClientKey
fn client_key() -> ClientKey {
    CLIENT_KEY.clone()
}

/// Represents an encrypted fixed-point value with `I` integer bits and `F` fractional bits.
#[derive(Clone)]
pub struct FheFixed<const I: usize, const F: usize> {
    ciphertext: Ciphertext,
}

impl<const I: usize, const F: usize> FheFixed<I, F> {
    /// Encrypt a raw fixed-point number `value` with the global client key.
    pub fn encrypt(value: f64) -> Self {
        // Scale by 2^F and clamp to [0, max_int]
        let scale = 1u64 << F;
        let max_int = ((1u128 << I) - 1) as u64;
        let int_val = (value * scale as f64)
            .round()
            .clamp(0.0, max_int as f64 * scale as f64) as u64;

        // Encrypt the integer
        let ct = client_key().encrypt(int_val);
        FheFixed { ciphertext: ct }
    }

    /// Decrypts and returns the raw floating-point approximation.
    pub fn decrypt(&self) -> f64 {
        // Decrypt back to integer
        let int_val: u64 = client_key().decrypt(&self.ciphertext);
        // Rescale to floating-point
        let scale = 1u64 << F;
        int_val as f64 / scale as f64
    }

    /// Access the inner ciphertext for advanced ops.
    pub fn inner(&self) -> &Ciphertext {
        &self.ciphertext
    }
}

// Homomorphic arithmetic operations via ServerKey
impl<const I: usize, const F: usize> Add for FheFixed<I, F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let ct = ServerKey::unchecked_add(&self.ciphertext, &rhs.ciphertext);
        FheFixed { ciphertext: ct }
    }
}

impl<const I: usize, const F: usize> Sub for FheFixed<I, F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let ct = ServerKey::unchecked_sub(&self.ciphertext, &rhs.ciphertext);
        FheFixed { ciphertext: ct }
    }
}

impl<const I: usize, const F: usize> Mul for FheFixed<I, F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let ct = ServerKey::unchecked_mul(&self.ciphertext, &rhs.ciphertext);
        FheFixed { ciphertext: ct }
    }
}

impl<const I: usize, const F: usize> Div for FheFixed<I, F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let ct = ServerKey::unchecked_div(&self.ciphertext, &rhs.ciphertext);
        FheFixed { ciphertext: ct }
    }
}

impl<const I: usize, const F: usize> Neg for FheFixed<I, F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let ct = ServerKey::unchecked_neg(&self.ciphertext);
        FheFixed { ciphertext: ct }
    }
}

// Comparison by decrypting and comparing in cleartext
impl<const I: usize, const F: usize> PartialEq for FheFixed<I, F> {
    fn eq(&self, other: &Self) -> bool {
        let l = self.decrypt();
        let r = other.decrypt();
        l == r
    }
}

impl<const I: usize, const F: usize> PartialOrd for FheFixed<I, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let l = self.decrypt();
        let r = other.decrypt();
        l.partial_cmp(&r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub_roundtrip() {
        let a = FheFixed::<8, 8>::encrypt(1.5);
        let b = FheFixed::<8, 8>::encrypt(2.0);
        let sum = a.clone() + b.clone();
        let diff = sum.clone() - a.clone();
        assert!((sum.decrypt() - 3.5).abs() < 1e-6);
        assert!((diff.decrypt() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn compare_roundtrip() {
        let a = FheFixed::<8, 8>::encrypt(2.25);
        let b = FheFixed::<8, 8>::encrypt(3.00);
        assert!(b > a);
        assert!(a < b);
        let c = FheFixed::<8, 8>::encrypt(2.25);
        assert!(a == c);
    }
}
