use std::num::Wrapping;
use std::mem;
use std::hash::Hasher;
use utils::*;

const K0: Wrapping<u64> = Wrapping(0xD6D018F5);
const K1: Wrapping<u64> = Wrapping(0xA2AA033B);
const K2: Wrapping<u64> = Wrapping(0x62992FC1);
const K3: Wrapping<u64> = Wrapping(0x30BC5B29);

pub struct MetroHash64 {
    v: [Wrapping<u64>; 4],
    b: [u64; 4],
    vseed: Wrapping<u64>,
    bytes: usize,
}

impl Default for MetroHash64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MetroHash64 {
    #[inline]
    pub fn new() -> MetroHash64 {
        Self::with_seed(0)
    }

    #[inline]
    pub fn with_seed(seed: u64) -> MetroHash64 {
        let vseed = (Wrapping(seed) + K2) * K0;
        MetroHash64 {
            b: unsafe { mem::uninitialized() },
            v: [vseed; 4],
            bytes: 0,
            vseed: vseed,
        }
    }
}

impl Hasher for MetroHash64 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut ptr = bytes.as_ptr() as usize;
        let end = ptr + bytes.len();
        // input buffer may be partially filled
        if self.bytes % 32 != 0 {
            let mut fill = 32 - (self.bytes % 32);
            if fill > bytes.len() {
                fill = bytes.len();
            }

            unsafe {
                copy_32(
                    ptr as *const u8,
                    (&mut self.b as *mut _ as *mut u8).offset((self.bytes % 32) as isize),
                    fill,
                );
            }
            ptr += fill;
            self.bytes += fill;

            // input buffer is still partially filled
            if self.bytes % 32 != 0 {
                return;
            }

            // process full input buffer
            self.v[0] = self.v[0] + (read_u64(&self.b[0] as *const _ as usize) * K0);
            self.v[0] = rotate_right(self.v[0], 29) + self.v[2];
            self.v[1] = self.v[1] + (read_u64(&self.b[1] as *const _ as usize) * K1);
            self.v[1] = rotate_right(self.v[1], 29) + self.v[3];
            self.v[2] = self.v[2] + (read_u64(&self.b[2] as *const _ as usize) * K2);
            self.v[2] = rotate_right(self.v[2], 29) + self.v[0];
            self.v[3] = self.v[3] + (read_u64(&self.b[3] as *const _ as usize) * K3);
            self.v[3] = rotate_right(self.v[3], 29) + self.v[1];
        }

        // bulk update
        self.bytes += end - ptr;
        while ptr + 32 <= end {
            // process directly from the source, bypassing the input buffer
            // these reads may be unaligned
            self.v[0] = self.v[0] + (read_u64_unaligned(ptr) * K0);
            ptr += 8;
            self.v[0] = rotate_right(self.v[0], 29) + self.v[2];
            self.v[1] = self.v[1] + (read_u64_unaligned(ptr) * K1);
            ptr += 8;
            self.v[1] = rotate_right(self.v[1], 29) + self.v[3];
            self.v[2] = self.v[2] + (read_u64_unaligned(ptr) * K2);
            ptr += 8;
            self.v[2] = rotate_right(self.v[2], 29) + self.v[0];
            self.v[3] = self.v[3] + (read_u64_unaligned(ptr) * K3);
            ptr += 8;
            self.v[3] = rotate_right(self.v[3], 29) + self.v[1];
        }

        // store remaining self.bytes in input buffer
        if ptr < end {
            unsafe {
                copy_32(
                    ptr as *const u8,
                    &mut self.b as *mut _ as *mut u8,
                    end - ptr,
                );
            }
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        // copy internal state
        let mut v = self.v;

        // finalize bulk loop, if used
        if self.bytes >= 32 {
            v[2] = v[2] ^ (rotate_right(((v[0] + v[3]) * K0) + v[1], 37) * K1);
            v[3] = v[3] ^ (rotate_right(((v[1] + v[2]) * K1) + v[0], 37) * K0);
            v[0] = v[0] ^ (rotate_right(((v[0] + v[2]) * K0) + v[3], 37) * K1);
            v[1] = v[1] ^ (rotate_right(((v[1] + v[3]) * K1) + v[2], 37) * K0);

            v[0] = self.vseed + (v[0] ^ v[1]);
        }

        // process any self.bytes remaining in the input buffer
        let mut ptr = &self.b as *const _ as usize;
        let end = ptr + self.bytes % 32;

        if (end - ptr) >= 16 {
            v[1] = v[0] + (read_u64(ptr) * K2);
            ptr += 8;
            v[1] = rotate_right(v[1], 29) * K3;
            v[2] = v[0] + (read_u64(ptr) * K2);
            ptr += 8;
            v[2] = rotate_right(v[2], 29) * K3;
            v[1] = v[1] ^ (rotate_right(v[1] * K0, 21) + v[2]);
            v[2] = v[2] ^ (rotate_right(v[2] * K3, 21) + v[1]);
            v[0] = v[0] + (v[2]);
        }

        if (end - ptr) >= 8 {
            v[0] = v[0] + (read_u64(ptr) * K3);
            ptr += 8;
            v[0] = v[0] ^ (rotate_right(v[0], 55) * K1);
        }

        if (end - ptr) >= 4 {
            v[0] = v[0] + (read_u32(ptr) * K3);
            ptr += 4;
            v[0] = v[0] ^ (rotate_right(v[0], 26) * K1);
        }

        if (end - ptr) >= 2 {
            v[0] = v[0] + (read_u16(ptr) * K3);
            ptr += 2;
            v[0] = v[0] ^ (rotate_right(v[0], 48) * K1);
        }

        if (end - ptr) >= 1 {
            v[0] = v[0] + (read_u8(ptr) * K3);
            v[0] = v[0] ^ (rotate_right(v[0], 37) * K1);
        }

        v[0] = v[0] ^ (rotate_right(v[0], 28));
        v[0] = v[0] * K0;
        v[0] = v[0] ^ (rotate_right(v[0], 29));

        v[0].0
    }
}
