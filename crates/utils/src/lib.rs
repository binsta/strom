#![cfg_attr(not(feature = "std"), no_std)]
use alloc::vec::{self, Vec};

pub fn uint_vec<T>(len: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);
    unsafe { vec.set_len(len) };
    return vec;
}
