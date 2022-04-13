use fxhash::FxHasher;
use core::hash::{Hasher, Hash};
use criterion::{criterion_group, criterion_main, black_box};
use criterion::Criterion as Bencher;


fn hash_bytes<H: Hasher>(mut s: H, x: &[u8]) -> u64 {
    Hasher::write(&mut s, x);
    s.finish()
}

fn hash_with<H: Hasher, T: Hash>(mut st: H, x: &T) -> u64 {
    x.hash(&mut st);
    st.finish()
}

fn hash<T: Hash>(x: &T) -> u64 {
    hash_with(FxHasher::default(), x)
}


fn bench_str_under_8_bytes(b: &mut Bencher) {
    let s = "foo";
    b.bench_function("fx bench_str_under_8_bytes", |b| {
        b.iter(|| {
            hash(&s);
        })
    });
}


fn bench_str_of_8_bytes(b: &mut Bencher) {
    let s = "foobar78";
    b.bench_function("fx bench_str_of_8_bytes", |b| {
        b.iter(|| {
            hash(&s);
        })
    });
}


fn bench_str_over_8_bytes(b: &mut Bencher) {
    let s = "foobarbaz0";
    b.bench_function("fx bench_str_over_8_bytes", |b| {
        b.iter(|| {
            hash(&s);
        })
    });
}


fn bench_long_str(b: &mut Bencher) {
    let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor \
             incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
             exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute \
             irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
             pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui \
             officia deserunt mollit anim id est laborum.";

    b.bench_function("fx bench_long_str", |b| {
        b.iter(|| {
            hash(&s);
        })
    });
}


fn bench_u32(b: &mut Bencher) {
    let u = 162629500u32;
    let u = black_box(u);
    b.bench_function("fx bench_u32", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_u32_keyed(b: &mut Bencher) {
    let u = 162629500u32;
    let u = black_box(u);
    b.bench_function("fx bench_u32_keyed", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_u64(b: &mut Bencher) {
    let u = 16262950014981195938u64;
    let u = black_box(u);
    b.bench_function("fx bench_u64", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_4(b: &mut Bencher) {
    let u = black_box([b' '; 4]);
    b.bench_function("fx bench_bytes_4", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_7(b: &mut Bencher) {
    let u = black_box([b' '; 7]);
    b.bench_function("fx bench_bytes_7", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_8(b: &mut Bencher) {
    let u = black_box([b' '; 8]);
    b.bench_function("fx bench_bytes_8", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_a_16(b: &mut Bencher) {
    let u = black_box([b' '; 16]);
    b.bench_function("fx bench_bytes_a_16", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_b_32(b: &mut Bencher) {
    let u = black_box([b' '; 32]);
    b.bench_function("fx bench_bytes_b_32", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


fn bench_bytes_c_128(b: &mut Bencher) {
    let u = black_box([b' '; 128]);
    b.bench_function("fx bench_bytes_c_128", |b| {
        b.iter(|| {
            hash(&u);
        })
    });
}


criterion_group!(benches, bench_str_under_8_bytes, bench_str_of_8_bytes, bench_str_over_8_bytes, bench_long_str, bench_u32 ,bench_u32_keyed , bench_u64, bench_bytes_4, bench_bytes_7, bench_bytes_8, bench_bytes_a_16, bench_bytes_b_32, bench_bytes_c_128);

criterion_main!(benches);