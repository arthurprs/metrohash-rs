use super::{MetroHash64, MetroHash128};
use std::mem::{self, size_of};
use std::fmt::Write;
use std::hash::Hasher;

const TEST_KEY_63: &'static [u8] = b"012345678901234567890123456789012345678901234567890123456789012";

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
impl_test!(test_metrohash64_0, MetroHash64, 0, [0x6Bu8, 0x75, 0x3D, 0xAE, 0x06, 0x70, 0x4B, 0xAD]);
// seed = 1
impl_test!(test_metrohash64_1, MetroHash64, 1, [0x3Bu8, 0x0D, 0x48, 0x1C, 0xF4, 0xB9, 0xB8, 0xDF]);
// seed = 0
impl_test!(test_metrohash128_0, MetroHash128, 0, [0xC7u8, 0x7C, 0xE2, 0xBF, 0xA4, 0xED, 0x9F, 0x9B,
                                                /*0x05, 0x48, 0xB2, 0xAC, 0x50, 0x74, 0xA2, 0x97*/]);
// seed = 1
impl_test!(test_metrohash128_1, MetroHash128, 1, [0x45u8, 0xA3, 0xCD, 0xB8, 0x38, 0x19, 0x9D, 0x7F,
                                                /*0xBD, 0xD6, 0x8D, 0x86, 0x7A, 0x14, 0xEC, 0xEF*/]);

extern crate test;
extern crate fnv;
extern crate twox_hash;

use std::hash::SipHasher;

fn hasher_bench<H>(b: &mut test::Bencher, mut hasher: H, len: usize)
    where H: Hasher
{
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| {
        hasher.write(&bytes);
        hasher.finish()
    });
}

fn siphash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, SipHasher::new(), len)
}

fn fnvhash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, <fnv::FnvHasher as Default>::default(), len)
}

fn metrohash64_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, MetroHash64::new(), len)
}

fn xxhash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, twox_hash::XxHash::with_seed(0), len)
}

#[bench]
fn siphash_megabyte(b: &mut test::Bencher) {
    siphash_bench(b, 1024 * 1024)
}

#[bench]
fn siphash_1024_byte(b: &mut test::Bencher) {
    siphash_bench(b, 1024)
}

#[bench]
fn siphash_512_byte(b: &mut test::Bencher) {
    siphash_bench(b, 512)
}

#[bench]
fn siphash_256_byte(b: &mut test::Bencher) {
    siphash_bench(b, 256)
}

#[bench]
fn siphash_128_byte(b: &mut test::Bencher) {
    siphash_bench(b, 128)
}

#[bench]
fn siphash_32_byte(b: &mut test::Bencher) {
    siphash_bench(b, 32)
}

#[bench]
fn siphash_16_byte(b: &mut test::Bencher) {
    siphash_bench(b, 16)
}

#[bench]
fn siphash_4_byte(b: &mut test::Bencher) {
    siphash_bench(b, 4)
}

#[bench]
fn siphash_1_byte(b: &mut test::Bencher) {
    siphash_bench(b, 1)
}

#[bench]
fn siphash_0_byte(b: &mut test::Bencher) {
    siphash_bench(b, 0)
}

#[bench]
fn fnvhash_megabyte(b: &mut test::Bencher) {
    fnvhash_bench(b, 1024 * 1024)
}

#[bench]
fn fnvhash_1024_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 1024)
}

#[bench]
fn fnvhash_512_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 512)
}

#[bench]
fn fnvhash_256_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 256)
}

#[bench]
fn fnvhash_128_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 128)
}

#[bench]
fn fnvhash_32_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 32)
}

#[bench]
fn fnvhash_16_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 16)
}

#[bench]
fn fnvhash_4_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 4)
}

#[bench]
fn fnvhash_1_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 1)
}

#[bench]
fn fnvhash_0_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, 0)
}

#[bench]
fn metrohash64_megabyte(b: &mut test::Bencher) {
    metrohash64_bench(b, 1024 * 1024)
}

#[bench]
fn metrohash64_1024_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 1024)
}

#[bench]
fn metrohash64_512_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 512)
}

#[bench]
fn metrohash64_256_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 256)
}

#[bench]
fn metrohash64_128_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 128)
}

#[bench]
fn metrohash64_32_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 32)
}

#[bench]
fn metrohash64_16_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 16)
}

#[bench]
fn metrohash64_4_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 4)
}

#[bench]
fn metrohash64_1_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 1)
}

#[bench]
fn metrohash64_0_byte(b: &mut test::Bencher) {
    metrohash64_bench(b, 0)
}

// #[bench]
// fn metrohash128_megabyte(b: &mut test::Bencher) { metrohash128_bench(b, 1024*1024) }

// #[bench]
// fn metrohash128_1024_byte(b: &mut test::Bencher) { metrohash128_bench(b, 1024) }

// #[bench]
// fn metrohash128_512_byte(b: &mut test::Bencher) { metrohash128_bench(b, 512) }

// #[bench]
// fn metrohash128_256_byte(b: &mut test::Bencher) { metrohash128_bench(b, 256) }

// #[bench]
// fn metrohash128_128_byte(b: &mut test::Bencher) { metrohash128_bench(b, 128) }

// #[bench]
// fn metrohash128_32_byte(b: &mut test::Bencher) { metrohash128_bench(b, 32) }

// #[bench]
// fn metrohash128_16_byte(b: &mut test::Bencher) { metrohash128_bench(b, 16) }

// #[bench]
// fn metrohash128_4_byte(b: &mut test::Bencher) { metrohash128_bench(b, 4) }

// #[bench]
// fn metrohash128_1_byte(b: &mut test::Bencher) { metrohash64_bench(b, 1) }

// #[bench]
// fn metrohash128_0_byte(b: &mut test::Bencher) { metrohash64_bench(b, 0) }

#[bench]
fn xxhash_megabyte(b: &mut test::Bencher) {
    xxhash_bench(b, 1024 * 1024)
}

#[bench]
fn xxhash_1024_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 1024)
}

#[bench]
fn xxhash_512_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 512)
}

#[bench]
fn xxhash_256_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 256)
}

#[bench]
fn xxhash_128_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 128)
}

#[bench]
fn xxhash_32_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 32)
}

#[bench]
fn xxhash_16_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 16)
}

#[bench]
fn xxhash_4_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 4)
}

#[bench]
fn xxhash_1_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 1)
}

#[bench]
fn xxhash_0_byte(b: &mut test::Bencher) {
    xxhash_bench(b, 0)
}
