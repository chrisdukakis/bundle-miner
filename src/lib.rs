/*
Permission is hereby granted, perpetual, worldwide, non-exclusive, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:



1. The Software cannot be used in any form or in any substantial portions for development, maintenance and for any other purposes, in the military sphere and in relation to military products, including, but not limited to:

a. any kind of armored force vehicles, missile weapons, warships, artillery weapons, air military vehicles (including military aircrafts, combat helicopters, military drones aircrafts), air defense systems, rifle armaments, small arms, firearms and side arms, melee weapons, chemical weapons, weapons of mass destruction;

b. any special software for development technical documentation for military purposes;

c. any special equipment for tests of prototypes of any subjects with military purpose of use;

d. any means of protection for conduction of acts of a military nature;

e. any software or hardware for determining strategies, reconnaissance, troop positioning, conducting military actions, conducting special operations;

f. any dual-use products with possibility to use the product in military purposes;

g. any other products, software or services connected to military activities;

h. any auxiliary means related to abovementioned spheres and products.



2. The Software cannot be used as described herein in any connection to the military activities. A person, a company, or any other entity, which wants to use the Software, shall take all reasonable actions to make sure that the purpose of use of the Software cannot be possibly connected to military purposes.



3. The Software cannot be used by a person, a company, or any other entity, activities of which are connected to military sphere in any means. If a person, a company, or any other entity, during the period of time for the usage of Software, would engage in activities, connected to military purposes, such person, company, or any other entity shall immediately stop the usage of Software and any its modifications or alterations.



4. Abovementioned restrictions should apply to all modification, alteration, merge, and to other actions, related to the Software, regardless of how the Software was changed due to the abovementioned actions.



The above copyright notice and this permission notice shall be included in all copies or substantial portions, modifications and alterations of the Software.



THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use iota_trytes::num::{int2trits};
use iota_crypto::{Kerl, Sponge};

pub const TRYTE_WIDTH: usize = 3;
pub const MAX_TRYTE_VALUE: i8 = 13;
pub const MIN_TRYTE_VALUE: i8 = -13;
pub const NUMBER_OF_SECURITY_LEVELS: usize = 3;
pub const NORMALIZED_FRAGMENT_LENGTH: usize = 27;
pub const NORMALIZED_BUNDLE_LENGTH: usize = NORMALIZED_FRAGMENT_LENGTH * NUMBER_OF_SECURITY_LEVELS;
pub const HASH_LENGTH: usize = 243;

pub const OBSOLETE_TAG_OFFSET: usize = 243 + 81;
pub const OBSOLETE_TAG_LENGTH: usize = 81;

#[wasm_bindgen]
pub fn min_normalized_bundle(a: &[i8], b: &[i8], out: &mut [i8]) {
    for i in 0..a.len() {
        if MAX_TRYTE_VALUE - a[i] < MAX_TRYTE_VALUE - b[i] {
            out[i] = a[i]
        } else {
            out[i] = b[i]
        }
    }
}

#[wasm_bindgen]
pub fn probability_of_losing(nb: &[i8], n: usize) -> f64 {
    let mut p = 0.0;

    for i in 0..n * NORMALIZED_FRAGMENT_LENGTH {

        let pi = 1.0 - ((MAX_TRYTE_VALUE - nb[i]) as f64 / (MAX_TRYTE_VALUE - MIN_TRYTE_VALUE + 1) as f64);

        p *= pi
    }

    return p
}

#[wasm_bindgen]
pub fn security_level(p: f64, radix: f64) -> f64 {
    (1.0 / p).ln() / radix.ln()
}

#[wasm_bindgen]
pub fn mine(min: &[i8], number_of_fragments: usize, essence: &mut [i8], offset: i32, count: i32) -> i32 {
    let mut index = offset;

    let mut best = 1.0;
    let mut best_index = 0;

    while index < offset + count {
        int2trits(index as i64, &mut essence[OBSOLETE_TAG_OFFSET..OBSOLETE_TAG_OFFSET + OBSOLETE_TAG_LENGTH]);

        let mut sponge = Kerl::default();
        let mut b: [i8; HASH_LENGTH] = [0; HASH_LENGTH];
        sponge.absorb(&essence);
        sponge.squeeze(&mut b);

        let mut nb: [i8; NORMALIZED_BUNDLE_LENGTH] = [MIN_TRYTE_VALUE; NORMALIZED_BUNDLE_LENGTH];
        normalized_bundle(&b, &mut nb);

        if nb.contains(&MAX_TRYTE_VALUE) == false {
            let mut min_b: [i8; NORMALIZED_BUNDLE_LENGTH] = [MIN_TRYTE_VALUE; NORMALIZED_BUNDLE_LENGTH];
            min_normalized_bundle(min, &nb, &mut min_b);

            let p = probability_of_losing(&min_b, number_of_fragments);
            if p < best {
                best = p;
                best_index = index
            }

        }

        index += 1;
    }

    return best_index;
}

#[wasm_bindgen]
pub fn normalized_bundle(bundle: &[i8], output: &mut [i8]) {
    for i in 0..NUMBER_OF_SECURITY_LEVELS {
        let mut sum: i64 = 0;
        for j in i * NORMALIZED_FRAGMENT_LENGTH..(i + 1) * NORMALIZED_FRAGMENT_LENGTH {
            output[j] =
                bundle[j * TRYTE_WIDTH] + bundle[j * TRYTE_WIDTH + 1] * 3 + bundle[j * TRYTE_WIDTH + 2] * 9;
            sum += output[j] as i64;
        }

        if sum >= 0 {
            while sum > 0 {
                for j in i * NORMALIZED_FRAGMENT_LENGTH..(i + 1) * NORMALIZED_FRAGMENT_LENGTH {
                    if output[j] > MIN_TRYTE_VALUE {
                        output[j] -= 1;
                        break;
                    }
                }
                sum -= 1;
            }
        } else {
            while sum < 0 {
                for j in i * NORMALIZED_FRAGMENT_LENGTH..(i + 1) * NORMALIZED_FRAGMENT_LENGTH {
                    if output[j] < MAX_TRYTE_VALUE {
                        output[j] += 1;
                        break;
                    }
                }
                sum += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iota_conversion::Trinary;

    const N: usize = 2;

    #[test]
    fn test_mine() {
        let v1: Vec<i8> = "QVXRKNRXFZIPFPREXRAPNHNSRFFQOWBGCAFZEGFCKDPDXRNVZQ9VJPQPPTFXKPVZVAIENQLETXRVSFKFO".trits();
        let mut a: [i8; HASH_LENGTH] = [0; HASH_LENGTH];
        for (i, t) in v1.iter().enumerate() {
            a[i] = *t;
        }

        let v2: Vec<i8> = "JKHLAKTRTDIKMTERIRYEWI9PPOJAKHZEMNCXFB9GTRZRWKSFVAZANHSPABGGQIJAVULKMPPAL9VBSRB9E".trits();
        let mut b: [i8; HASH_LENGTH] = [0; HASH_LENGTH];
        for (i, t) in v2.iter().enumerate() {
            b[i] = *t;
        }

        let mut nb1: [i8; NORMALIZED_BUNDLE_LENGTH] = [0; NORMALIZED_BUNDLE_LENGTH];
        normalized_bundle(&a, &mut nb1);

        let mut nb2: [i8; NORMALIZED_BUNDLE_LENGTH] = [0; NORMALIZED_BUNDLE_LENGTH];
        normalized_bundle(&b, &mut nb2);

        let mut m: [i8; NORMALIZED_BUNDLE_LENGTH] = [0; NORMALIZED_BUNDLE_LENGTH];
        min_normalized_bundle(&nb1, &nb2, &mut m);

        const E: usize = 486 * 4;
        let mut e: [i8; E] = [0; E];

        let i: i32 = mine(&m, N, &mut e, 0, 1000000);

        assert_eq!(i, 722);
    }
}
