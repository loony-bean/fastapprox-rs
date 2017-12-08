#[inline]
pub fn to_bits(x: f32) -> u32 {
    unsafe { ::std::mem::transmute::<f32, u32>(x) }
}

#[inline]
pub fn from_bits(x: u32) -> f32 {
    unsafe { ::std::mem::transmute::<u32, f32>(x) }
}
