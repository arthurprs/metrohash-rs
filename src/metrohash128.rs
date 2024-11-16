use crate::utils::*;
use std::hash::Hasher;
use std::mem::MaybeUninit;
use std::num::Wrapping;

const K0: Wrapping<u64> = Wrapping(0xC83A91E1);
const K1: Wrapping<u64> = Wrapping(0x8648DBDB);
const K2: Wrapping<u64> = Wrapping(0x7BDEC03B);
const K3: Wrapping<u64> = Wrapping(0x2F5870A5);

pub struct MetroHash128 {
    v: [Wrapping<u64>; 4],
    b: [MaybeUninit<u64>; 4],
    bytes: usize,
}

impl Default for MetroHash128 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MetroHash128 {
    #[inline]
    pub fn new() -> MetroHash128 {
        Self::with_seed(0)
    }

    #[inline]
    pub fn with_seed(seed: u64) -> MetroHash128 {
        let seed = Wrapping(seed);
        MetroHash128 {
            b: [MaybeUninit::uninit(); 4],
            v: [
                (seed - K0) * K3,
                (seed + K1) * K2,
                (seed + K0) * K2,
                (seed - K1) * K3,
            ],
            bytes: 0,
        }
    }

    #[inline]
    pub fn finish128(&self) -> (u64, u64) {
        unsafe {
            // copy internal state
            let mut v = self.v;

            // finalize bulk loop, if used
            if self.bytes >= 32 {
                v[2] ^= rotate_right(((v[0] + v[3]) * K0) + v[1], 21) * K1;
                v[3] ^= rotate_right(((v[1] + v[2]) * K1) + v[0], 21) * K0;
                v[0] = v[0] ^ (rotate_right(((v[0] + v[2]) * K0) + v[3], 21) * K1);
                v[1] = v[1] ^ (rotate_right(((v[1] + v[3]) * K1) + v[2], 21) * K0);
            }

            // process any self.bytes remaining in the input buffer
            let mut ptr = self.b.as_ptr().cast::<u8>();
            let end = ptr.add(self.bytes % 32);

            if end.offset_from(ptr) >= 16 {
                v[0] += read_u64(ptr) * K2;
                ptr = ptr.add(8);
                v[0] = rotate_right(v[0], 33) * K3;
                v[1] += read_u64(ptr) * K2;
                ptr = ptr.add(8);
                v[1] = rotate_right(v[1], 33) * K3;
                v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 45) * K1);
                v[1] = v[1] ^ (rotate_right((v[1] * K3) + v[0], 45) * K0);
            }

            if end.offset_from(ptr) >= 8 {
                v[0] += read_u64(ptr) * K2;
                ptr = ptr.add(8);
                v[0] = rotate_right(v[0], 33) * K3;
                v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 27) * K1);
            }

            if end.offset_from(ptr) >= 4 {
                v[1] += read_u32(ptr) * K2;
                ptr = ptr.add(4);
                v[1] = rotate_right(v[1], 33) * K3;
                v[1] = v[1] ^ (rotate_right((v[1] * K3) + v[0], 46) * K0);
            }

            if end.offset_from(ptr) >= 2 {
                v[0] += read_u16(ptr) * K2;
                ptr = ptr.add(2);
                v[0] = rotate_right(v[0], 33) * K3;
                v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 22) * K1);
            }

            if end.offset_from(ptr) >= 1 {
                v[1] += read_u8(ptr) * K2;
                v[1] = rotate_right(v[1], 33) * K3;
                v[1] = v[1] ^ (rotate_right((v[1] * K3) + v[0], 58) * K0);
            }

            v[0] = v[0] + (rotate_right((v[0] * K0) + v[1], 13));
            v[1] = v[1] + (rotate_right((v[1] * K1) + v[0], 37));
            v[0] = v[0] + (rotate_right((v[0] * K2) + v[1], 13));
            v[1] = v[1] + (rotate_right((v[1] * K3) + v[0], 37));

            (v[0].0, v[1].0)
        }
    }
}

impl Hasher for MetroHash128 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            let mut ptr = bytes.as_ptr();
            let end = ptr.add(bytes.len());
            // input buffer may be partially filled
            if self.bytes % 32 != 0 {
                let mut fill = 32 - (self.bytes % 32);
                if fill > bytes.len() {
                    fill = bytes.len();
                }

                copy_32(
                    ptr,
                    self.b[0].as_mut_ptr().cast::<u8>().add(self.bytes % 32),
                    fill,
                );

                ptr = ptr.add(fill);
                self.bytes += fill;

                // input buffer is still partially filled
                if self.bytes % 32 != 0 {
                    return;
                }

                // process full input buffer
                self.v[0] += read_u64(&self.b[0] as *const _) * K0;
                self.v[0] = rotate_right(self.v[0], 29) + self.v[2];
                self.v[1] += read_u64(&self.b[1] as *const _) * K1;
                self.v[1] = rotate_right(self.v[1], 29) + self.v[3];
                self.v[2] += read_u64(&self.b[2] as *const _) * K2;
                self.v[2] = rotate_right(self.v[2], 29) + self.v[0];
                self.v[3] += read_u64(&self.b[3] as *const _) * K3;
                self.v[3] = rotate_right(self.v[3], 29) + self.v[1];
            }

            // bulk update
            self.bytes += end.offset_from(ptr) as usize;
            while end.offset_from(ptr) >= 32 {
                // process directly from the source, bypassing the input buffer
                // these reads may be unaligned
                self.v[0] += read_u64_unaligned(ptr) * K0;
                ptr = ptr.add(8);
                self.v[0] = rotate_right(self.v[0], 29) + self.v[2];
                self.v[1] += read_u64_unaligned(ptr) * K1;
                ptr = ptr.add(8);
                self.v[1] = rotate_right(self.v[1], 29) + self.v[3];
                self.v[2] += read_u64_unaligned(ptr) * K2;
                ptr = ptr.add(8);
                self.v[2] = rotate_right(self.v[2], 29) + self.v[0];
                self.v[3] += read_u64_unaligned(ptr) * K3;
                ptr = ptr.add(8);
                self.v[3] = rotate_right(self.v[3], 29) + self.v[1];
            }

            // store remaining self.bytes in input buffer
            if ptr < end {
                copy_32(
                    ptr,
                    self.b.as_mut_ptr().cast::<u8>(),
                    end.offset_from(ptr) as usize,
                );
            }
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.finish128().0
    }
}
