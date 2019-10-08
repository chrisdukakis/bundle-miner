extern crate bundle_miner;
use bundle_miner::*;
use iota_conversion::Trinary;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

const N: usize = 2;
const L: usize = N * NORMALIZED_FRAGMENT_LENGTH;
const MIN: [i8; L] = [MIN_TRYTE_VALUE; L];
const MAX: [i8; L] = [MAX_TRYTE_VALUE; L];

#[wasm_bindgen_test]
fn test_min_normalized_bundle() {
    let mut a: [i8; L] = [0; L];
    let mut b: [i8; L] = [0; L];

    a[0] = 13;
    b[3] = 12;

    let mut expected: [i8; L] = [0; L];
    expected[0] = 13;
    expected[3] = 12;

    let mut actual: [i8; L] = [0; L];
    min_normalized_bundle(&a, &b, &mut actual);


    assert!(expected.iter().zip(actual.iter()).all(|(a,b)| a == b));
}

#[wasm_bindgen_test]
fn test_probability_of_losing() {
    assert_eq!(probability_of_losing(&MIN, N), 0.0, "probability for max number of rounds is 0.");
    assert_eq!(probability_of_losing(&MAX, N), 1.0, "probability for min number of rounds is 1.");
}

#[wasm_bindgen_test]
fn test_security_level() {
    assert_eq!(security_level(probability_of_losing(&MAX, N), 3.0), 0.0, "security level for min number of rounds is 0.");
}

#[wasm_bindgen_test]
fn test_normalized_bundle() {
    let v: Vec<i8> = "QVXRKNRXFZIPFPREXRAPNHNSRFFQOWBGCAFZEGFCKDPDXRNVZQ9VJPQPPTFXKPVZVAIENQLETXRVSFKFO".trits();
	let mut b: [i8; HASH_LENGTH] = [0; HASH_LENGTH];
    for (i, t) in v.iter().enumerate() {
        b[i] = *t;
    }

	let expected: [i8; NORMALIZED_BUNDLE_LENGTH] = [
		13,13,13,13,13,-2,-9,-3,6,-1,9,-11,6,-11,-9,5,-3,-9,1,-11,-13,8,-13,-8,-9,6,
		6,13,-9,-4,2,7,3,1,6,-1,5,7,6,3,11,4,-11,4,-3,-9,-13,-5,-1,-10,0,-5,10,-11,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
	];
	let mut actual: [i8; NORMALIZED_BUNDLE_LENGTH] = [0; NORMALIZED_BUNDLE_LENGTH];

    normalized_bundle(&b, &mut actual);

    assert!(expected[0..L].iter().zip(actual[0..L].iter()).all(|(a,b)| a == b));
}

#[wasm_bindgen_test]
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

    let i: i32 = mine(&m, N, &mut e, 0, 1000);

    assert_eq!(i, 722);
}
