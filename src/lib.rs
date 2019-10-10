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

        let pi = 1.0 - ((MAX_TRYTE_VALUE - nb[i]) as f64 / (MAX_TRYTE_VALUE - MIN_TRYTE_VALUE) as f64);

        if pi > 0.0 {
            if p == 0.0 {
                p = 1.0
            }

            p *= pi
        }
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
