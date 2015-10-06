# Rust MetroHash

[![Build Status](https://travis-ci.org/arthurprs/metrohash-rs.svg)](https://travis-ci.org/arthurprs/metrohash-rs)

Rust implementation of MetroHash.

MetroHash is a high quality, high performance hash algorithm, always faster than the famous XXHash64. Although for small keys (up to 16 bytes) you probably want to use FNV hash instead.

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
test tests::fnvhash_0_byte        ... bench:           0 ns/iter (+/- 0)
test tests::fnvhash_1024_byte     ... bench:       1,411 ns/iter (+/- 116) = 725 MB/s
test tests::fnvhash_128_byte      ... bench:         188 ns/iter (+/- 7) = 680 MB/s
test tests::fnvhash_16_byte       ... bench:          23 ns/iter (+/- 2) = 695 MB/s
test tests::fnvhash_1_byte        ... bench:           1 ns/iter (+/- 0) = 1000 MB/s
test tests::fnvhash_256_byte      ... bench:         351 ns/iter (+/- 27) = 729 MB/s
test tests::fnvhash_32_byte       ... bench:          44 ns/iter (+/- 3) = 727 MB/s
test tests::fnvhash_4_byte        ... bench:           6 ns/iter (+/- 1) = 666 MB/s
test tests::fnvhash_512_byte      ... bench:         738 ns/iter (+/- 61) = 693 MB/s
test tests::fnvhash_megabyte      ... bench:   1,490,245 ns/iter (+/- 83,206) = 703 MB/s
test tests::metrohash64_0_byte    ... bench:          12 ns/iter (+/- 1)
test tests::metrohash64_1024_byte ... bench:         161 ns/iter (+/- 12) = 6360 MB/s
test tests::metrohash64_128_byte  ... bench:          34 ns/iter (+/- 3) = 3764 MB/s
test tests::metrohash64_16_byte   ... bench:          22 ns/iter (+/- 2) = 727 MB/s
test tests::metrohash64_1_byte    ... bench:          23 ns/iter (+/- 3) = 43 MB/s
test tests::metrohash64_256_byte  ... bench:          52 ns/iter (+/- 5) = 4923 MB/s
test tests::metrohash64_32_byte   ... bench:          19 ns/iter (+/- 1) = 1684 MB/s
test tests::metrohash64_4_byte    ... bench:          23 ns/iter (+/- 2) = 173 MB/s
test tests::metrohash64_512_byte  ... bench:          85 ns/iter (+/- 7) = 6023 MB/s
test tests::metrohash64_megabyte  ... bench:     146,649 ns/iter (+/- 9,034) = 7150 MB/s
test tests::siphash_0_byte        ... bench:          18 ns/iter (+/- 1)
test tests::siphash_1024_byte     ... bench:         813 ns/iter (+/- 50) = 1259 MB/s
test tests::siphash_128_byte      ... bench:         119 ns/iter (+/- 7) = 1075 MB/s
test tests::siphash_16_byte       ... bench:          30 ns/iter (+/- 3) = 533 MB/s
test tests::siphash_1_byte        ... bench:          20 ns/iter (+/- 1) = 50 MB/s
test tests::siphash_256_byte      ... bench:         214 ns/iter (+/- 16) = 1196 MB/s
test tests::siphash_32_byte       ... bench:          44 ns/iter (+/- 5) = 727 MB/s
test tests::siphash_4_byte        ... bench:          26 ns/iter (+/- 1) = 153 MB/s
test tests::siphash_512_byte      ... bench:         419 ns/iter (+/- 33) = 1221 MB/s
test tests::siphash_megabyte      ... bench:     824,892 ns/iter (+/- 59,512) = 1270 MB/s
test tests::xxhash_0_byte         ... bench:          16 ns/iter (+/- 1)
test tests::xxhash_1024_byte      ... bench:         200 ns/iter (+/- 16) = 5120 MB/s
test tests::xxhash_128_byte       ... bench:          42 ns/iter (+/- 4) = 3047 MB/s
test tests::xxhash_16_byte        ... bench:          28 ns/iter (+/- 2) = 571 MB/s
test tests::xxhash_1_byte         ... bench:          28 ns/iter (+/- 2) = 35 MB/s
test tests::xxhash_256_byte       ... bench:          65 ns/iter (+/- 5) = 3938 MB/s
test tests::xxhash_32_byte        ... bench:          26 ns/iter (+/- 2) = 1230 MB/s
test tests::xxhash_4_byte         ... bench:          27 ns/iter (+/- 3) = 148 MB/s
test tests::xxhash_512_byte       ... bench:         113 ns/iter (+/- 12) = 4530 MB/s
test tests::xxhash_megabyte       ... bench:     185,998 ns/iter (+/- 14,585) = 5637 MB/s
```

Also: [1](http://imgur.com/olhGqhU) and [2](http://imgur.com/V8evli2)
