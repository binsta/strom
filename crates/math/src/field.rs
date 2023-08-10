// SPDX-License-Identifier: MIT

// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// MIT License for more details.

// You should have received a copy of the MIT License
// along with this program. If not, see <https://www.mit.org/licenses/>.
//! Feild Constant Use for ZkSTARK.
//!
//! This Feild constant use for Computing .
#![cfg_attr(not(feature = "std"), no_std)]
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use std::ops::Range;

/// Constant for Feild Modules
/// Feild modules = 2^128 - 45 * 2^40 + 1
pub const M: u128 = 340282366920938463463374557953744961537;

/// Constant use Feild Modules
/// Feild Modules = 2^40 root for uint
pub const G: u128 = 23953097886125630542083529559205016746;

/// Public Constant
pub const MODULUS: u128 = M;
pub const RANGE: Range<u128> = Range { start: 0, end: M };
pub const ZERO: u128 = 0;
pub const ONE: u128 = 1;

// Basic Arithmetic

/// Computing (a + b) % m; a and b are assumed to be value feild elements.
pub fn add(a: u128, b: u128) -> u128 {
    let z = M - b;
    if a < z {
        M - z + a
    } else {
        a - z
    }
}

/// Computing (a -b) % m; a and b are assumed to value feild elements
pub fn sub(a: u128, b: u128) -> u128 {
    if a < b {
        M - b + a
    } else {
        a - b
    }
}

///Computing (a- b) % m; a and b are assumed to be value feild elements.
pub fn mul(a: u128, b: u128) -> u128 {
    let (x0, x1, x2) = mul_128x64(a, ((b >> 64) as u64).into()); // x = a * b_hi
    let (mut x0, mut x1, x2) = mul_reduce(x0, x1, x2); // x = x - (x >> 128) * m
    if x2 == 1 {
        // if there was an overflow beyond 128 bits, subtract
        // modulus from the result to make sure it fits into
        // 128 bits; this can potentially be removed in favor
        // of checking overflow later
        let (t0, t1) = sub_modulus(x0, x1); // x = x - m
        x0 = t0;
        x1 = t1;
    }

    let (y0, y1, y2) = mul_128x64(a, (b as u64).into()); // y = a * b_lo

    let (mut y1, carry) = add64_with_carry(y1, x0, 0); // y = y + (x << 64)
    let (mut y2, y3) = add64_with_carry(y2, x1, carry);
    if y3 == 1 {
        // if there was an overflow beyond 192 bits, subtract
        // modulus * 2^64 from the result to make sure it fits
        // into 192 bits; this can potentially replace the
        // previous overflow check (but needs to be proven)
        let (t0, t1) = sub_modulus(y1, y2); // y = y - (m << 64)
        y1 = t0;
        y2 = t1;
    }

    let (mut z0, mut z1, z2) = mul_reduce(y0, y1, y2); // z = y - (y >> 128) * m

    // make sure z is smaller than m
    if z2 == 1 || (z1 == (M >> 64) as u64 && z0 >= (M as u64)) {
        let (t0, t1) = sub_modulus(z0, z1); // z = z - m
        z0 = t0;
        z1 = t1;
    }

    return ((z1 as u128) << 64) + (z0 as u128);
}

pub fn rand() -> u128 {
    let range = Uniform::from(RANGE);
    let mut g = thread_rng();
    return g.sample(range);
}

#[inline(always)]
fn mul_128x64(a: u128, b: u128) -> (u64, u64, u64) {
    let z_lo = ((a as u64) as u128) * (b as u128);
    let z_hi = (a >> 64) * (b as u128);
    let z_hi = z_hi + (z_lo >> 64);
    return (z_lo as u64, z_hi as u64, (z_hi >> 64) as u64);
}

#[inline(always)]
fn sub_modulus(a_lo: u64, a_hi: u64) -> (u64, u64) {
    let mut z = 0u128.wrapping_sub(M);
    z = z.wrapping_add(a_lo as u128);
    z = z.wrapping_add((a_hi as u128) << 64);
    return (z as u64, (z >> 64) as u64);
}
#[inline(always)]
pub const fn add64_with_carry(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    return (ret as u64, (ret >> 64) as u64);
}

#[inline(always)]
fn mul_reduce(z0: u64, z1: u64, z2: u64) -> (u64, u64, u64) {
    let (q0, q1, q2) = mul_by_modulus(z2);
    let (z0, z1, z2) = sub_192x192(z0, z1, z2, q0, q1, q2);
    return (z0, z1, z2);
}

#[inline(always)]
fn mul_by_modulus(a: u64) -> (u64, u64, u64) {
    let a_lo = (a as u128).wrapping_mul(M);
    let a_hi = if a == 0 { 0 } else { a - 1 };
    return (a_lo as u64, (a_lo >> 64) as u64, a_hi);
}

#[inline(always)]
fn sub_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128).wrapping_sub(b0 as u128);
    let z1 = (a1 as u128).wrapping_sub((b1 as u128) + (z0 >> 127));
    let z2 = (a2 as u128).wrapping_sub((b2 as u128) + (z1 >> 127));
    return (z0 as u64, z1 as u64, z2 as u64);
}

#[cfg(test)]
mod test {
    use num_bigint::BigUint;
    use std::convert::TryInto;

    #[test]
    fn add() {
        let r: u128 = super::rand();
        assert_eq!(r, super::add(r, 0));

        // test addition within bounds
        assert_eq!(5, super::add(2, 3));

        // test overflow
        let m: u128 = super::MODULUS;
        let t = m - 1;
        assert_eq!(0, super::add(t, 1));
        assert_eq!(1, super::add(t, 2));

        // test random values
        let r1: u128 = super::rand();
        let r2: u128 = super::rand();

        let expected = (BigUint::from(r1) + BigUint::from(r2)) % BigUint::from(super::M);
        let expected = u128::from_le_bytes((expected.to_bytes_le()[..]).try_into().unwrap());
        assert_eq!(expected, super::add(r1, r2));
    }
}
