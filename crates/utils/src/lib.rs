#![cfg_attr(not(feature = "std"), no_std)]
use std::vec::Vec;

pub fn uint_vec<T>(len: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);
    unsafe { vec.set_len(len) };
    return vec;
}

pub fn filled_vec<T: Copy>(length: usize, capacity: usize, value: T) -> Vec<T> {
    let mut vector = vec![value; capacity];
    vector.truncate(length);
    return vector;
}
