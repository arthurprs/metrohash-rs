#![cfg_attr(test, feature(test))]
pub extern crate test;
#[cfg(feature = "fnv")]
pub extern crate fnv;
#[cfg(feature = "twox-hash")]
pub extern crate twox_hash;
extern crate metrohash;

pub use std::hash::Hasher;
pub use std::hash::SipHasher;

pub fn hasher_bench<H>(b: &mut test::Bencher, len: usize)
    where H: Hasher + Default
{
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| {
               let mut hasher: H = Default::default();
               hasher.write(&bytes);
               hasher.finish()
           });
}

macro_rules! impl_bench {
    ($mod_name: ident, $hasher: ty) => (
        mod $mod_name {
            use super::*;

            // #[bench]
            // fn hash_1_mega_bytes(b: &mut test::Bencher) {
            //     hasher_bench::<$hasher>(b, 1024 * 1024)
            // }
            //
            // #[bench]
            // fn hash_1_kilo_bytes(b: &mut test::Bencher) {
            //     hasher_bench::<$hasher>(b, 1024)
            // }
            //
            // #[bench]
            // fn hash_512_bytes(b: &mut test::Bencher) {
            //     hasher_bench::<$hasher>(b, 512)
            // }

            #[bench]
            fn hash_256_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 256)
            }

            #[bench]
            fn hash_128_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 128)
            }

            #[bench]
            fn hash_032_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 32)
            }

            #[bench]
            fn hash_016_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 16)
            }

            #[bench]
            fn hash_009_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 9)
            }

            #[bench]
            fn hash_008_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 8)
            }

            #[bench]
            fn hash_007_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 7)
            }

            #[bench]
            fn hash_004_bytes(b: &mut test::Bencher) {
                hasher_bench::<$hasher>(b, 4)
            }
        }
    );
}

impl_bench!(std_sip, SipHasher);
impl_bench!(metro64, metrohash::MetroHash64);
// avoid bloat, performance is virtually the same
// impl_bench!(metro128, metrohash::MetroHash128);
#[cfg(feature = "fnv")]
impl_bench!(fnvh, fnv::FnvHasher);
#[cfg(feature = "twox-hash")]
impl_bench!(xxh, twox_hash::XxHash);
