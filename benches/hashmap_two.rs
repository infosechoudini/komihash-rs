// This benchmark suite contains some benchmarks along a set of dimensions:
//   Hasher: std default (SipHash) and crate default (AHash).
//   Int key distribution: low bit heavy, top bit heavy, and random.
//   Task: basic functionality: insert, insert_erase, lookup, lookup_fail, iter
#![feature(test)]

extern crate test;
extern crate seahash;
extern crate fxhash;

use test::{black_box, Bencher};

//use hashbrown::hash_map::DefaultHashBuilder;
//use hashbrown::{HashMap, HashSet};
use std::{
    sync::atomic::{self, AtomicUsize},
};
use std::collections::{HashMap, HashSet};
use komihash_rs::hash_map::KomiHashMap;
use std::hash::BuildHasherDefault;

const SIZE: usize = 3000;

// The default hashmap when using this crate directly.
//type AHashMap<K, V> = HashMap<K, V, DefaultHashBuilder>;
use ahash::AHashMap as Ahashmap;

type AHashMap<K, V> = Ahashmap< K, V>;

// This uses the hashmap from this crate with the default hasher of the stdlib.
type KomiHM<K, V> = KomiHashMap<K, V>;

type FXHashMap< K, V> = HashMap< K, V, FXBuildHasher>;

/// A builder for default SeaHasher hashers.
type FXBuildHasher = BuildHasherDefault<fxhash::FxHasher>;

type SeaHashMap< K, V> = HashMap< K, V, SeaBuildHasher>;

/// A builder for default SeaHasher hashers.
type SeaBuildHasher = BuildHasherDefault<seahash::SeaHasher>;


// A random key iterator.
#[derive(Clone, Copy)]
struct RandomKeys {
    state: usize,
}

impl RandomKeys {
    fn new() -> Self {
        RandomKeys { state: 0 }
    }
}

impl Iterator for RandomKeys {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        // Add 1 then multiply by some 32 bit prime.
        self.state = self.state.wrapping_add(1).wrapping_mul(3_787_392_781);
        Some(self.state)
    }
}

// Just an arbitrary side effect to make the maps not shortcircuit to the non-dropping path
// when dropping maps/entries (most real world usages likely have drop in the key or value)
lazy_static::lazy_static! {
    static ref SIDE_EFFECT: AtomicUsize = AtomicUsize::new(0);
}

#[derive(Clone)]
struct DropType(usize);
impl Drop for DropType {
    fn drop(&mut self) {
        SIDE_EFFECT.fetch_add(self.0, atomic::Ordering::SeqCst);
    }
}

macro_rules! bench_suite {
    ($bench_macro:ident, $bench_ahash_serial:ident, $bench_komi_serial:ident,
        $bench_sea_serial:ident, $bench_fx_serial:ident,
     $bench_ahash_highbits:ident, $bench_komi_highbits:ident, $bench_sea_highbits:ident, $bench_fx_highbits:ident,
     $bench_ahash_random:ident, $bench_komi_random:ident, $bench_sea_random:ident, $bench_fx_random:ident) => {
        $bench_macro!($bench_ahash_serial, AHashMap, 0..);
        $bench_macro!($bench_komi_serial, KomiHM, 0..);
        $bench_macro!($bench_sea_serial, SeaHashMap, 0..);
        $bench_macro!($bench_fx_serial, FXHashMap, 0..);
        $bench_macro!(
            $bench_ahash_highbits,
            AHashMap,
            (0..).map(usize::swap_bytes)
        );
        $bench_macro!(
            $bench_komi_highbits,
            KomiHM,
            (0..).map(usize::swap_bytes)
        );
        $bench_macro!(
            $bench_sea_highbits,
            SeaHashMap,
            (0..).map(usize::swap_bytes)
        );
        $bench_macro!(
            $bench_fx_highbits,
            FXHashMap,
            (0..).map(usize::swap_bytes)
        );
        $bench_macro!($bench_ahash_random, AHashMap, RandomKeys::new());
        $bench_macro!($bench_komi_random, KomiHM, RandomKeys::new());
        $bench_macro!($bench_sea_random, SeaHashMap, RandomKeys::new());
        $bench_macro!($bench_fx_random, FXHashMap, RandomKeys::new());
    };
}

macro_rules! bench_insert {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m = $maptype::with_capacity_and_hasher(SIZE, Default::default());
            b.iter(|| {
                m.clear();
                for i in ($keydist).take(SIZE) {
                    m.insert(i, (DropType(i), [i; 20]));
                }
                black_box(&mut m);
            });
            eprintln!("{}", SIDE_EFFECT.load(atomic::Ordering::SeqCst));
        }
    };
}

bench_suite!(
    bench_insert,
    insert_ahash_serial,
    insert_komi_serial,
    insert_sea_serial,
    insert_fx_serial,
    insert_ahash_highbits,
    insert_komi_highbits,
    insert_sea_highbits,
    insert_fx_highbits,
    insert_ahash_random,
    insert_komi_random,
    insert_sea_random,
    insert_fx_random
);

macro_rules! bench_grow_insert {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut m = $maptype::default();
                for i in ($keydist).take(SIZE) {
                    m.insert(i, DropType(i));
                }
                black_box(&mut m);
            })
        }
    };
}

/*
bench_suite!(
    bench_grow_insert,
    grow_insert_ahash_serial,
    grow_insert_komi_serial,
    grow_insert_ahash_highbits,
    grow_insert_komi_highbits,
    grow_insert_ahash_random,
    grow_insert_komi_random
);
*/
macro_rules! bench_insert_erase {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut base = $maptype::default();
            for i in ($keydist).take(SIZE) {
                base.insert(i, DropType(i));
            }
            let skip = $keydist.skip(SIZE);
            b.iter(|| {
                let mut m = base.clone();
                let mut add_iter = skip.clone();
                let mut remove_iter = $keydist;
                // While keeping the size constant,
                // replace the first keydist with the second.
                for (add, remove) in (&mut add_iter).zip(&mut remove_iter).take(SIZE) {
                    m.insert(add, DropType(add));
                    black_box(m.remove(&remove));
                }
                black_box(m);
            });
            eprintln!("{}", SIDE_EFFECT.load(atomic::Ordering::SeqCst));
        }
    };
}

/*
bench_suite!(
    bench_insert_erase,
    insert_erase_ahash_serial,
    insert_erase_komi_serial,
    insert_erase_ahash_highbits,
    insert_erase_komi_highbits,
    insert_erase_ahash_random,
    insert_erase_komi_random
);
*/

macro_rules! bench_lookup {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m = $maptype::default();
            for i in $keydist.take(SIZE) {
                m.insert(i, DropType(i));
            }

            b.iter(|| {
                for i in $keydist.take(SIZE) {
                    black_box(m.get(&i));
                }
            });
            eprintln!("{}", SIDE_EFFECT.load(atomic::Ordering::SeqCst));
        }
    };
}

/*
bench_suite!(
    bench_lookup,
    lookup_ahash_serial,
    lookup_komi_serial,
    lookup_ahash_highbits,
    lookup_komi_highbits,
    lookup_ahash_random,
    lookup_komi_random
);
*/

macro_rules! bench_lookup_fail {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m = $maptype::default();
            let mut iter = $keydist;
            for i in (&mut iter).take(SIZE) {
                m.insert(i, DropType(i));
            }

            b.iter(|| {
                for i in (&mut iter).take(SIZE) {
                    black_box(m.get(&i));
                }
            })
        }
    };
}

/*
bench_suite!(
    bench_lookup_fail,
    lookup_fail_ahash_serial,
    lookup_fail_komi_serial,
    lookup_fail_ahash_highbits,
    lookup_fail_komi_highbits,
    lookup_fail_ahash_random,
    lookup_fail_komi_random
);
*/

macro_rules! bench_iter {
    ($name:ident, $maptype:ident, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m = $maptype::default();
            for i in ($keydist).take(SIZE) {
                m.insert(i, DropType(i));
            }

            b.iter(|| {
                for i in &m {
                    black_box(i);
                }
            })
        }
    };
}

/*
bench_suite!(
    bench_iter,
    iter_ahash_serial,
    iter_komi_serial,
    iter_ahash_highbits,
    iter_komi_highbits,
    iter_ahash_random,
    iter_komi_random
);
*/
#[bench]
fn clone_small(b: &mut Bencher) {
    let mut m = HashMap::new();
    for i in 0..10 {
        m.insert(i, DropType(i));
    }

    b.iter(|| {
        black_box(m.clone());
    })
}

#[bench]
fn clone_from_small(b: &mut Bencher) {
    let mut m = HashMap::new();
    let mut m2 = HashMap::new();
    for i in 0..10 {
        m.insert(i, DropType(i));
    }

    b.iter(|| {
        m2.clone_from(&m);
        black_box(&mut m2);
    })
}

#[bench]
fn clone_large(b: &mut Bencher) {
    let mut m = HashMap::new();
    for i in 0..1000 {
        m.insert(i, DropType(i));
    }

    b.iter(|| {
        black_box(m.clone());
    })
}

#[bench]
fn clone_from_large(b: &mut Bencher) {
    let mut m = HashMap::new();
    let mut m2 = HashMap::new();
    for i in 0..1000 {
        m.insert(i, DropType(i));
    }

    b.iter(|| {
        m2.clone_from(&m);
        black_box(&mut m2);
    })
}

#[bench]
fn rehash_in_place(b: &mut Bencher) {
    b.iter(|| {
        let mut set = HashSet::new();

        // Each loop triggers one rehash
        for _ in 0..10 {
            for i in 0..224 {
                set.insert(i);
            }

            assert_eq!(
                set.capacity(),
                224,
                "The set must be at or close to capacity to trigger a re hashing"
            );

            for i in 100..1400 {
                set.remove(&(i - 100));
                set.insert(i);
            }
            set.clear();
        }
    });
}