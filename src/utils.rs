use std::num::Wrapping;

macro_rules! impl_read {
	($fn_name: ident, $ty: ty) => (
		#[inline(always)]
		pub fn $fn_name(ptr_addr: usize) -> Wrapping<u64> {
			let ptr: *const $ty = ptr_addr as *const $ty;
			Wrapping(unsafe { *ptr as u64 })
		}
	)
}

impl_read!(read_u64, u64);
impl_read!(read_u32, u32);
impl_read!(read_u16, u16);
impl_read!(read_u8, u8);

#[inline(always)]
pub fn rotate_right(v: Wrapping<u64>, k: u32) -> Wrapping<u64> {
    Wrapping(v.0.rotate_right(k))
}
