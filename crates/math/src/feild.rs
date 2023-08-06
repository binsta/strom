// SPDX-License-Identifier: MIT

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Feild Constant Use for ZkSTARK.
//!
//! This Feild constant use for Computing .
#![cfg_attr(not(feature = "std"), no_std)]
use rand::distributions::{Distribution, Uniform};
use std::convert::TryInto;
use std::ops::Range;

/// Constant for Feild Modules
/// Feild modules = 2^128 - 45 * 2^40 + 1
pub const M: u128 = 340282366920938463463374557953744961537;

/// Constant use Feild Modules
/// Feild Modules = 2^40 root for uint
pub const G: u128 = 23953097886125630542083529559205016746;

/// Public Constant
pub const MODULES: u128 = M;
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
    let (x0, x1, x2) = mul_128x64(a, (b >> 64) as u64); // x = a * b_hi
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

    let (y0, y1, y2) = mul_128x64(a, b as u64); // y = a * b_lo

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
