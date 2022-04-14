// Credit = https://github.com/tkaitchuck/aHash/blob/master/src/convert.rs
// Transmute covert properties works faster than using u*::to_le_bytes()

pub(crate) trait Convert<To> {
    fn convert(self) -> To;
}

macro_rules! convert {
    ($a:ty, $b:ty) => {
        impl Convert<$b> for $a {
            #[inline(always)]
            fn convert(self) -> $b {
                unsafe {
                    core::mem::transmute::<$a, $b>(self)
                }
            }
        }
        impl Convert<$a> for $b {
            #[inline(always)]
            fn convert(self) -> $a {
                unsafe {
                    core::mem::transmute::<$b, $a>(self)
                }
            }
        }
    };
}

convert!([u128; 4], [u64; 8]);
convert!([u128; 4], [u32; 16]);
convert!([u128; 4], [u16; 32]);
convert!([u128; 4], [u8; 64]);
convert!([u128; 2], [u64; 4]);
convert!([u128; 2], [u32; 8]);
convert!([u128; 2], [u16; 16]);
convert!([u128; 2], [u8; 32]);
convert!(u128, [u64; 2]);
convert!(u128, [u32; 4]);
convert!(u128, [u16; 8]);
convert!(u128, [u8; 16]);
convert!([u64; 8], [u32; 16]);
convert!([u64; 8], [u16; 32]);
convert!([u64; 8], [u8; 64]);
convert!([u64; 4], [u32; 8]);
convert!([u64; 4], [u16; 16]);
convert!([u64; 4], [u8; 32]);
convert!([u64; 2], [u32; 4]);
convert!([u64; 2], [u16; 8]);
convert!([u64; 2], [u8; 16]);
convert!([u32; 4], [u16; 8]);
convert!([u32; 4], [u8; 16]);
convert!([u16; 8], [u8; 16]);
convert!(u64, [u32; 2]);
convert!(u64, [u16; 4]);
convert!(u64, [u8; 8]);
convert!([u32; 2], [u16; 4]);
convert!([u32; 2], [u8; 8]);
convert!(u32, [u16; 2]);
convert!(u32, [u8; 4]);
convert!([u16; 2], [u8; 4]);
convert!(u16, [u8; 2]);


//Added target for Usize 
#[cfg(target_pointer_width = "64")]
convert!(usize, [u8; 8]);
#[cfg(target_pointer_width = "32")]
convert!(usize, [u8; 4]);



convert!([[u64; 4]; 2], [u8; 64]);

convert!([f64; 2], [u8; 16]);
convert!([f32; 4], [u8; 16]);
convert!(f64, [u8; 8]);
convert!([f32; 2], [u8; 8]);
convert!(f32, [u8; 4]);
