use super::{MetroHash64, MetroHash128};
use std::mem::{self, size_of};
use std::fmt::Write;
use std::hash::Hasher;

const TEST_KEY_63: &'static [u8] =
    b"012345678901234567890123456789012345678901234567890123456789012";

fn hexlify<T>(value: &T) -> String {
    let mut result = String::with_capacity(size_of::<T>());
    unsafe {
        let value_bytes: *const u8 = mem::transmute(value);
        for i in 0..result.capacity() {
            write!(result, "{:02X}", *value_bytes.offset(i as isize)).unwrap();
        }
    }
    result
}

macro_rules! impl_test {
	($test_fn: ident, $test_hasher: ident, $seed: expr, $expected: expr) => (
		#[test]
		fn $test_fn() {
			let mut hasher = $test_hasher::with_seed($seed);
			hasher.write(TEST_KEY_63);
			let hash = hasher.finish();
			assert_eq!(hexlify(&hash), hexlify(&$expected));
		}
	)
}

// seed = 0
impl_test!(test_metrohash64_0,
           MetroHash64,
           0,
           [0x6Bu8, 0x75, 0x3D, 0xAE, 0x06, 0x70, 0x4B, 0xAD]);
// seed = 1
impl_test!(test_metrohash64_1,
           MetroHash64,
           1,
           [0x3Bu8, 0x0D, 0x48, 0x1C, 0xF4, 0xB9, 0xB8, 0xDF]);
// seed = 0
impl_test!(test_metrohash128_0,
           MetroHash128,
           0,
           [0xC7u8,
            0x7C,
            0xE2,
            0xBF,
            0xA4,
            0xED,
            0x9F,
            0x9B
            /*0x05, 0x48, 0xB2, 0xAC, 0x50, 0x74, 0xA2, 0x97*/]);
// seed = 1
impl_test!(test_metrohash128_1,
           MetroHash128,
           1,
           [0x45u8,
            0xA3,
            0xCD,
            0xB8,
            0x38,
            0x19,
            0x9D,
            0x7F
            /*0xBD, 0xD6, 0x8D, 0x86, 0x7A, 0x14, 0xEC, 0xEF*/]);
