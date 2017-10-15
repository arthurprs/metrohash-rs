# Rust MetroHash

[![crates.io](https://img.shields.io/crates/v/metrohash.svg)](https://crates.io/crates/metrohash)
[![Build Status](https://travis-ci.org/arthurprs/metrohash-rs.svg)](https://travis-ci.org/arthurprs/metrohash-rs)

MetroHash is a high quality, high performance hash algorithm.

It hashes small keys in the single digit nanoseconds and surpasses 10+ GB/s in medium to long length keys.

Since the hash quality is also excellent (passes SMhasher), it's a good alternative in any situation where cryptographic security is not required.

# Usage

```rust
extern crate metrohash;
use metrohash::MetroHashMap

let mut hash: = MetroHashMap::default();
hash.insert(1000, "1000");
assert_eq!(hash.get(&1000), Some(&"1000"));
```

# Benchmarks

```
test metro64::hash_u64          ... bench:           7 ns/iter (+/- 0) = 1142 MB/s
test metro64::hash_004_bytes    ... bench:           7 ns/iter (+/- 0) = 571 MB/s
test metro64::hash_007_bytes    ... bench:           8 ns/iter (+/- 0) = 875 MB/s
test metro64::hash_008_bytes    ... bench:           7 ns/iter (+/- 0) = 1142 MB/s
test metro64::hash_009_bytes    ... bench:           7 ns/iter (+/- 6) = 1285 MB/s
test metro64::hash_012_bytes    ... bench:           7 ns/iter (+/- 1) = 1714 MB/s
test metro64::hash_016_bytes    ... bench:           9 ns/iter (+/- 0) = 1777 MB/s
test metro64::hash_032_bytes    ... bench:          11 ns/iter (+/- 0) = 2909 MB/s
test metro64::hash_128_bytes    ... bench:          19 ns/iter (+/- 1) = 6736 MB/s
test metro64::hash_256_bytes    ... bench:          28 ns/iter (+/- 9) = 9142 MB/s
test metro64::hash_512_bytes    ... bench:          49 ns/iter (+/- 5) = 10448 MB/s
test metro64::hash_1_kilo_bytes ... bench:          88 ns/iter (+/- 36) = 11636 MB/s
test fnvh::hash_u64             ... bench:           3 ns/iter (+/- 2) = 2666 MB/s
test fnvh::hash_004_bytes       ... bench:           1 ns/iter (+/- 0) = 4000 MB/s
test fnvh::hash_007_bytes       ... bench:           3 ns/iter (+/- 0) = 2333 MB/s
test fnvh::hash_008_bytes       ... bench:           3 ns/iter (+/- 0) = 2666 MB/s
test fnvh::hash_009_bytes       ... bench:           3 ns/iter (+/- 0) = 3000 MB/s
test fnvh::hash_012_bytes       ... bench:           5 ns/iter (+/- 0) = 2400 MB/s
test fnvh::hash_016_bytes       ... bench:           7 ns/iter (+/- 0) = 2285 MB/s
test fnvh::hash_032_bytes       ... bench:          17 ns/iter (+/- 1) = 1882 MB/s
test fnvh::hash_128_bytes       ... bench:         125 ns/iter (+/- 9) = 1024 MB/s
test fnvh::hash_256_bytes       ... bench:         280 ns/iter (+/- 34) = 914 MB/s
test fnvh::hash_512_bytes       ... bench:         589 ns/iter (+/- 36) = 869 MB/s
test fnvh::hash_1_kilo_bytes    ... bench:       1,192 ns/iter (+/- 43) = 859 MB/s
test std_sip::hash_u64          ... bench:          16 ns/iter (+/- 0) = 500 MB/s
test std_sip::hash_004_bytes    ... bench:          12 ns/iter (+/- 3) = 333 MB/s
test std_sip::hash_007_bytes    ... bench:          14 ns/iter (+/- 0) = 500 MB/s
test std_sip::hash_008_bytes    ... bench:          15 ns/iter (+/- 1) = 533 MB/s
test std_sip::hash_009_bytes    ... bench:          17 ns/iter (+/- 9) = 529 MB/s
test std_sip::hash_012_bytes    ... bench:          16 ns/iter (+/- 0) = 750 MB/s
test std_sip::hash_016_bytes    ... bench:          19 ns/iter (+/- 2) = 842 MB/s
test std_sip::hash_032_bytes    ... bench:          27 ns/iter (+/- 2) = 1185 MB/s
test std_sip::hash_128_bytes    ... bench:          72 ns/iter (+/- 18) = 1777 MB/s
test std_sip::hash_256_bytes    ... bench:         133 ns/iter (+/- 13) = 1924 MB/s
test std_sip::hash_512_bytes    ... bench:         254 ns/iter (+/- 22) = 2015 MB/s
test std_sip::hash_1_kilo_bytes ... bench:         500 ns/iter (+/- 55) = 2048 MB/s
test xxh::hash_u64              ... bench:          11 ns/iter (+/- 0) = 727 MB/s
test xxh::hash_004_bytes        ... bench:          12 ns/iter (+/- 1) = 333 MB/s
test xxh::hash_007_bytes        ... bench:          20 ns/iter (+/- 1) = 350 MB/s
test xxh::hash_008_bytes        ... bench:          12 ns/iter (+/- 4) = 666 MB/s
test xxh::hash_009_bytes        ... bench:          19 ns/iter (+/- 5) = 473 MB/s
test xxh::hash_012_bytes        ... bench:          19 ns/iter (+/- 1) = 631 MB/s
test xxh::hash_016_bytes        ... bench:          13 ns/iter (+/- 0) = 1230 MB/s
test xxh::hash_032_bytes        ... bench:          17 ns/iter (+/- 4) = 1882 MB/s
test xxh::hash_128_bytes        ... bench:          25 ns/iter (+/- 1) = 5120 MB/s
test xxh::hash_256_bytes        ... bench:          36 ns/iter (+/- 2) = 7111 MB/s
test xxh::hash_512_bytes        ... bench:          57 ns/iter (+/- 28) = 8982 MB/s
test xxh::hash_1_kilo_bytes     ... bench:          98 ns/iter (+/- 7) = 10448 MB/s
```

# Links:
* [Author original post](http://www.jandrewrogers.com/2015/05/27/metrohash/)
* [Original implementation](https://github.com/jandrewrogers/MetroHash)
