
#![feature(test)]
extern crate test;
extern crate fnv;
extern crate fxhash;
extern crate seahash;
extern crate ahash;

use std::hash::{Hash, Hasher};
use test::{Bencher, black_box};
use komihash_rs::Komihash;

fn komihash<H: Hash>(b: H) -> u64 {
    let mut hasher = Komihash::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn fnvhash<H: Hash>(b: H) -> u64 {
    let mut hasher = fnv::FnvHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn seahash<H: Hash>(b: H) -> u64 {
    let mut hasher = seahash::SeaHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

fn ahash<H: Hash>(b: H) -> u64 {
    let mut hasher = ahash::AHasher::default();
    b.hash(&mut hasher);
    hasher.finish()
}

macro_rules! generate_benches {
    ($($fx:ident, $fx32:ident, $fx64:ident, $fnv:ident, $sea:ident, $komi:ident, $ahash:ident, $s:expr),* $(,)*) => (
        $(
            #[bench]
            fn $fx(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fxhash::hash(&s)
                })
            }

            #[bench]
            fn $fx32(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fxhash::hash32(&s)
                })
            }

            #[bench]
            fn $fx64(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fxhash::hash64(&s)
                })
            }

            #[bench]
            fn $fnv(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    fnvhash(&s)
                })
            }

            #[bench]
            fn $sea(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    seahash(&s)
                })
            }

            #[bench]
            fn $komi(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    komihash(&s)
                })
            }
            #[bench]
            fn $ahash(b: &mut Bencher) {
                let s = black_box($s);
                b.iter(|| {
                    ahash(&s)
                })
            }
        )*
    )
}

generate_benches!(
    bench_fx_003, bench_fx32_003, bench_fx64_003, bench_fnv_003, bench_seahash_003, bench_komihash_003, bench_ahash_003, "123",
    bench_fx_004, bench_fx32_004, bench_fx64_004, bench_fnv_004, bench_seahash_004, bench_komihash_004, bench_ahash_004,  "1234",
    bench_fx_011, bench_fx32_011, bench_fx64_011, bench_fnv_011, bench_seahash_011, bench_komihash_011, bench_ahash_011,  "12345678901",
    bench_fx_012, bench_fx32_012, bench_fx64_012, bench_fnv_012, bench_seahash_012, bench_komihash_012, bench_ahash_012,  "123456789012",
    bench_fx_023, bench_fx32_023, bench_fx64_023, bench_fnv_023, bench_seahash_023, bench_komihash_023, bench_ahash_023,  "12345678901234567890123",
    bench_fx_024, bench_fx32_024, bench_fx64_024, bench_fnv_024, bench_seahash_024, bench_komihash_024, bench_ahash_024,  "123456789012345678901234",
    bench_fx_064, bench_fx32_064, bench_fx64_064, bench_fnv_064, bench_seahash_064, bench_komihash_064, bench_ahash_064,  "1123456789012345678901234567890123456789012345678901234567890123",
    bench_fx_068, bench_fx32_068, bench_fx64_068, bench_fnv_068, bench_seahash_068, bench_komihash_068, bench_ahash_068,  "11234567890123456789012345678901234567890123456789012345678901234567",
    bench_fx_132, bench_fx32_132, bench_fx64_132, bench_fnv_132, bench_seahash_132, bench_komihash_132, bench_ahash_132,  "112345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901",
);