use rand::{ thread_rng, Rng };
use num_bigint::BigUint;

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.mod_pow(a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.mod_pow(a, p)
}

pub trait PowModulus {
    fn mod_pow(&self, pow: Self, modulus: Self) -> Self;
}

impl PowModulus for u64 {
    fn mod_pow(&self, pow: u64, modulus: u64) -> u64 {
        let base = BigUint::from(*self);
        let ans = base.modpow(&BigUint::from(pow), &BigUint::from(modulus));

        ans.iter_u64_digits().next().unwrap()
    }
}