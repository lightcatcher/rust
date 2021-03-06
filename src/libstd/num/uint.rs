// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for `uint`

use num;
use sys;

pub use self::generated::*;

uint_module!(uint, int, ::int::bits)

///
/// Divide two numbers, return the result, rounded up.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The smallest integer `q` such that `x/y <= q`.
///
pub fn div_ceil(x: uint, y: uint) -> uint {
    let div = x / y;
    if x % y == 0u { div }
    else { div + 1u }
}

///
/// Divide two numbers, return the result, rounded to the closest integer.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The integer `q` closest to `x/y`.
///
pub fn div_round(x: uint, y: uint) -> uint {
    let div = x / y;
    if x % y * 2u  < y { div }
    else { div + 1u }
}

///
/// Divide two numbers, return the result, rounded down.
///
/// Note: This is the same function as `div`.
///
/// # Arguments
///
/// * x - an integer
/// * y - an integer distinct from 0u
///
/// # Return value
///
/// The smallest integer `q` such that `x/y <= q`. This
/// is either `x/y` or `x/y + 1`.
///
pub fn div_floor(x: uint, y: uint) -> uint { return x / y; }

impl num::Times for uint {
    #[inline]
    ///
    /// A convenience form for basic repetition. Given a uint `x`,
    /// `do x.times { ... }` executes the given block x times.
    ///
    /// Equivalent to `for uint::range(0, x) |_| { ... }`.
    ///
    /// Not defined on all integer types to permit unambiguous
    /// use with integer literals of inferred integer-type as
    /// the self-value (eg. `do 100.times { ... }`).
    ///
    fn times(&self, it: &fn()) {
        let mut i = *self;
        while i > 0 {
            it();
            i -= 1;
        }
    }
}

/// Returns the smallest power of 2 greater than or equal to `n`
#[inline]
pub fn next_power_of_two(n: uint) -> uint {
    let halfbits: uint = sys::size_of::<uint>() * 4u;
    let mut tmp: uint = n - 1u;
    let mut shift: uint = 1u;
    while shift <= halfbits { tmp |= tmp >> shift; shift <<= 1u; }
    return tmp + 1u;
}

#[test]
fn test_next_power_of_two() {
    assert!((next_power_of_two(0u) == 0u));
    assert!((next_power_of_two(1u) == 1u));
    assert!((next_power_of_two(2u) == 2u));
    assert!((next_power_of_two(3u) == 4u));
    assert!((next_power_of_two(4u) == 4u));
    assert!((next_power_of_two(5u) == 8u));
    assert!((next_power_of_two(6u) == 8u));
    assert!((next_power_of_two(7u) == 8u));
    assert!((next_power_of_two(8u) == 8u));
    assert!((next_power_of_two(9u) == 16u));
    assert!((next_power_of_two(10u) == 16u));
    assert!((next_power_of_two(11u) == 16u));
    assert!((next_power_of_two(12u) == 16u));
    assert!((next_power_of_two(13u) == 16u));
    assert!((next_power_of_two(14u) == 16u));
    assert!((next_power_of_two(15u) == 16u));
    assert!((next_power_of_two(16u) == 16u));
    assert!((next_power_of_two(17u) == 32u));
    assert!((next_power_of_two(18u) == 32u));
    assert!((next_power_of_two(19u) == 32u));
    assert!((next_power_of_two(20u) == 32u));
    assert!((next_power_of_two(21u) == 32u));
    assert!((next_power_of_two(22u) == 32u));
    assert!((next_power_of_two(23u) == 32u));
    assert!((next_power_of_two(24u) == 32u));
    assert!((next_power_of_two(25u) == 32u));
    assert!((next_power_of_two(26u) == 32u));
    assert!((next_power_of_two(27u) == 32u));
    assert!((next_power_of_two(28u) == 32u));
    assert!((next_power_of_two(29u) == 32u));
    assert!((next_power_of_two(30u) == 32u));
    assert!((next_power_of_two(31u) == 32u));
    assert!((next_power_of_two(32u) == 32u));
    assert!((next_power_of_two(33u) == 64u));
    assert!((next_power_of_two(34u) == 64u));
    assert!((next_power_of_two(35u) == 64u));
    assert!((next_power_of_two(36u) == 64u));
    assert!((next_power_of_two(37u) == 64u));
    assert!((next_power_of_two(38u) == 64u));
    assert!((next_power_of_two(39u) == 64u));
}

#[test]
fn test_overflows() {
    use uint;
    assert!((uint::max_value > 0u));
    assert!((uint::min_value <= 0u));
    assert!((uint::min_value + uint::max_value + 1u == 0u));
}

#[test]
fn test_div() {
    assert!((div_floor(3u, 4u) == 0u));
    assert!((div_ceil(3u, 4u)  == 1u));
    assert!((div_round(3u, 4u) == 1u));
}

#[test]
pub fn test_times() {
    use num::Times;
    let ten = 10 as uint;
    let mut accum = 0;
    do ten.times { accum += 1; }
    assert!((accum == 10));
}
