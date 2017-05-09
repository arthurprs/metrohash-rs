# Rust MetroHash

[![crates.io](https://img.shields.io/crates/v/metrohash.svg)](https://crates.io/crates/metrohash)
[![Build Status](https://travis-ci.org/arthurprs/metrohash-rs.svg)](https://travis-ci.org/arthurprs/metrohash-rs)

Rust implementation of MetroHash.

MetroHash is a high quality, high performance hash algorithm.

It's mostly faster than stdlib SipHash and always faster than XXHash64. Although for small keys (up to 16 bytes) you probably want to use FNV hash instead.

The current implementation is a direct translation of the original code and passes all tests. It'd be nice to have more idiomatic code in the future.

For more information please check:
* [Author original post](http://www.jandrewrogers.com/2015/05/27/metrohash/)
* [Original implementation](https://github.com/jandrewrogers/MetroHash)

# Usage

```rust
extern crate metrohash;
use std::collections::HashMap;
use std::collections::hash_state::DefaultState;
use metrohash::MetroHash;

let mut hash: HashMap<_, _, DefaultState<MetroHash>> = Default::default();
hash.insert(1000, "1000");
assert_eq!(hash.get(&1000), Some(&"1000"));
```

# Benchmarks

```
test metro64::hash_004_bytes ... bench:           6 ns/iter (+/- 0) = 666 MB/s
test metro64::hash_007_bytes ... bench:          17 ns/iter (+/- 1) = 411 MB/s
test metro64::hash_008_bytes ... bench:           6 ns/iter (+/- 0) = 1333 MB/s
test metro64::hash_009_bytes ... bench:          15 ns/iter (+/- 1) = 600 MB/s
test metro64::hash_016_bytes ... bench:           8 ns/iter (+/- 0) = 2000 MB/s
test metro64::hash_032_bytes ... bench:          10 ns/iter (+/- 0) = 3200 MB/s
test metro64::hash_128_bytes ... bench:          18 ns/iter (+/- 0) = 7111 MB/s
test metro64::hash_256_bytes ... bench:          28 ns/iter (+/- 1) = 9142 MB/s
test std_sip::hash_004_bytes ... bench:          10 ns/iter (+/- 4) = 400 MB/s
test std_sip::hash_007_bytes ... bench:          13 ns/iter (+/- 3) = 538 MB/s
test std_sip::hash_008_bytes ... bench:          13 ns/iter (+/- 0) = 615 MB/s
test std_sip::hash_009_bytes ... bench:          14 ns/iter (+/- 1) = 642 MB/s
test std_sip::hash_016_bytes ... bench:          17 ns/iter (+/- 7) = 941 MB/s
test std_sip::hash_032_bytes ... bench:          24 ns/iter (+/- 1) = 1333 MB/s
test std_sip::hash_128_bytes ... bench:          70 ns/iter (+/- 4) = 1828 MB/s
test std_sip::hash_256_bytes ... bench:         136 ns/iter (+/- 23) = 1882 MB/s
test fnvh::hash_004_bytes    ... bench:           2 ns/iter (+/- 0) = 2000 MB/s
test fnvh::hash_007_bytes    ... bench:           3 ns/iter (+/- 0) = 2333 MB/s
test fnvh::hash_008_bytes    ... bench:           3 ns/iter (+/- 0) = 2666 MB/s
test fnvh::hash_009_bytes    ... bench:           4 ns/iter (+/- 1) = 2250 MB/s
test fnvh::hash_016_bytes    ... bench:           7 ns/iter (+/- 0) = 2285 MB/s
test fnvh::hash_032_bytes    ... bench:          18 ns/iter (+/- 1) = 1777 MB/s
test fnvh::hash_128_bytes    ... bench:         128 ns/iter (+/- 5) = 1000 MB/s
test fnvh::hash_256_bytes    ... bench:         286 ns/iter (+/- 28) = 895 MB/s
test xxh::hash_004_bytes     ... bench:          11 ns/iter (+/- 0) = 363 MB/s
test xxh::hash_007_bytes     ... bench:          20 ns/iter (+/- 2) = 350 MB/s
test xxh::hash_008_bytes     ... bench:          11 ns/iter (+/- 1) = 727 MB/s
test xxh::hash_009_bytes     ... bench:          18 ns/iter (+/- 1) = 500 MB/s
test xxh::hash_016_bytes     ... bench:          13 ns/iter (+/- 1) = 1230 MB/s
test xxh::hash_032_bytes     ... bench:          17 ns/iter (+/- 0) = 1882 MB/s
test xxh::hash_128_bytes     ... bench:          25 ns/iter (+/- 0) = 5120 MB/s
test xxh::hash_256_bytes     ... bench:          37 ns/iter (+/- 10) = 6918 MB/s
```
