#![cfg_attr(test, feature(test))]
pub extern crate test;
#[cfg(feature = "fnv")]
pub extern crate fnv;
#[cfg(feature = "twox-hash")]
pub extern crate twox_hash;
extern crate metrohash;

pub use std::hash::Hasher;
pub use std::hash::SipHasher;

pub fn hasher_bench<H>(b: &mut test::Bencher, mut hasher: H, len: usize)
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

fn metrohash64_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, metrohash::MetroHash64::new(), len)
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

#[cfg(feature = "fnv")]
mod feature_fnv {
	use super::*;

	fn fnvhash_bench(b: &mut test::Bencher, len: usize) {
	    hasher_bench(b, <fnv::FnvHasher as Default>::default(), len)
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

#[cfg(feature = "twox-hash")]
mod feature_twoxhash {
	use super::*;

	fn xxhash_bench(b: &mut test::Bencher, len: usize) {
	    hasher_bench(b, twox_hash::XxHash::with_seed(0), len)
	}

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
}
