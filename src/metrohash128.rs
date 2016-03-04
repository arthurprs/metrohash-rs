use std::num::Wrapping;
use std::mem;
use std::ptr;
use std::hash::Hasher;
use utils::*;

const K0: Wrapping<u64> = Wrapping(0xC83A91E1);
const K1: Wrapping<u64> = Wrapping(0x8648DBDB);
const K2: Wrapping<u64> = Wrapping(0x7BDEC03B);
const K3: Wrapping<u64> = Wrapping(0x2F5870A5);

pub struct MetroHash128 {
    v: [Wrapping<u64>; 4],
    b: [u64; 4],
    bytes: usize,
}

impl Default for MetroHash128 {
    fn default() -> Self {
        Self::new()
    }
}

impl MetroHash128 {
    pub fn new() -> MetroHash128 {
        Self::with_seed(0)
    }

    pub fn with_seed(seed: u64) -> MetroHash128 {
        let seed = Wrapping(seed);
        MetroHash128 {
            b: unsafe { mem::uninitialized() },
            v: [(seed - K0) * K3, (seed + K1) * K2, (seed + K0) * K2, (seed - K1) * K3],
            bytes: 0,
        }
    }

    fn finish128(&self) -> (u64, u64) {
        // copy internal state
        let mut v = self.v;

        // finalize bulk loop, if used
        if self.bytes >= 32 {
            v[2] = v[2] ^ (rotate_right(((v[0] + v[3]) * K0) + v[1], 21) * K1);
            v[3] = v[3] ^ (rotate_right(((v[1] + v[2]) * K1) + v[0], 21) * K0);
            v[0] = v[0] ^ (rotate_right(((v[0] + v[2]) * K0) + v[3], 21) * K1);
            v[1] = v[1] ^ (rotate_right(((v[1] + v[3]) * K1) + v[2], 21) * K0);
        }

        // process any self.bytes remaining in the input buffer
        let mut ptr = &self.b as *const _ as usize;
        let end = ptr + self.bytes % 32;

        if (end - ptr) >= 16 {
            v[0] = v[0] + (read_u64(ptr) * K2);
            ptr += 8;
            v[0] = rotate_right(v[0], 33) * K3;
            v[1] = v[1] + (read_u64(ptr) * K2);
            ptr += 8;
            v[1] = rotate_right(v[1], 33) * K3;
            v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 45) * K1);
            v[1] = v[1] ^ (rotate_right((v[1] * K3) + v[0], 45) * K0);
        }

        if (end - ptr) >= 8 {
            v[0] = v[0] + (read_u64(ptr) * K2);
            ptr += 8;
            v[0] = rotate_right(v[0], 33) * K3;
            v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 27) * K1);
        }

        if (end - ptr) >= 4 {
            v[1] = v[1] + (read_u32(ptr) * K2);
            ptr += 4;
            v[1] = rotate_right(v[1], 33) * K3;
            v[1] = v[1] ^ (rotate_right((v[1] * K3) + v[0], 46) * K0);
        }

        if (end - ptr) >= 2 {
            v[0] = v[0] + (read_u16(ptr) * K2);
            ptr += 2;
            v[0] = rotate_right(v[0], 33) * K3;
            v[0] = v[0] ^ (rotate_right((v[0] * K2) + v[1], 22) * K1);
        }

        if (end - ptr) >= 1 {
            v[1] = v[1] + (read_u8(ptr) * K2);
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

impl Hasher for MetroHash128 {

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
                ptr::copy_nonoverlapping(ptr as *const u8,
                                         (&mut self.b[0] as *mut _ as *mut u8)
                                             .offset((self.bytes % 32) as isize),
                                         fill);
            }
            ptr += fill;
            self.bytes += fill;

            // input buffer is still partially filled
            if self.bytes % 32 != 0 {
                return
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
            self.v[0] = self.v[0] + (read_u64(ptr) * K0);
            ptr += 8;
            self.v[0] = rotate_right(self.v[0], 29) + self.v[2];
            self.v[1] = self.v[1] + (read_u64(ptr) * K1);
            ptr += 8;
            self.v[1] = rotate_right(self.v[1], 29) + self.v[3];
            self.v[2] = self.v[2] + (read_u64(ptr) * K2);
            ptr += 8;
            self.v[2] = rotate_right(self.v[2], 29) + self.v[0];
            self.v[3] = self.v[3] + (read_u64(ptr) * K3);
            ptr += 8;
            self.v[3] = rotate_right(self.v[3], 29) + self.v[1];
        }

        // store remaining self.bytes in input buffer
        if ptr < end {
            unsafe {
                ptr::copy_nonoverlapping(ptr as *const u8, &mut self.b[0] as *mut _ as *mut u8, end - ptr);
            }
        }
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        self.finish128().0
    }
}
