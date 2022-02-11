#![feature(test)]

extern crate test;

use rustc_hash::FxHashMap;
use std::collections::HashMap;
use test::Bencher;

fn build_collection(n: u32) -> Vec<u16> {
    let mut prng = oorandom::Rand32::new(u64::from(std::process::id()));
    (0..n).map(|_i| prng.rand_range(0..n) as u16).collect()
}

fn build_lookup(n: u32) -> Vec<u16> {
    let mut prng = oorandom::Rand32::new(u64::from(!std::process::id()));
    (0..100).map(|_i| prng.rand_range(0..n) as u16).collect()
}

macro_rules! bench_vec {
    ($b:ident, $n:literal) => {
        let collection = build_collection($n);
        let lookup = build_lookup($n);
        $b.iter(|| {
            lookup.iter().for_each(|&i| {
                test::black_box(hashmap_vs_vec::vec(i, &collection));
            })
        });
    };
}

macro_rules! bench_map {
    ($b:ident, $n:literal, $map:ty, $f:path) => {
        let collection: $map = build_collection($n)
            .into_iter()
            .enumerate()
            .map(|(i, j)| (i as u16, j))
            .collect();
        let lookup = build_lookup($n);
        $b.iter(|| {
            lookup.iter().for_each(|&i| {
                test::black_box($f(i, &collection));
            })
        });
    };
}

#[bench]
fn bench_vec_10(b: &mut Bencher) {
    bench_vec!(b, 10);
}

#[bench]
fn bench_vec_50(b: &mut Bencher) {
    bench_vec!(b, 50);
}

#[bench]
fn bench_vec_250(b: &mut Bencher) {
    bench_vec!(b, 250);
}

#[bench]
fn bench_vec_500(b: &mut Bencher) {
    bench_vec!(b, 500);
}

#[bench]
fn bench_vec_1000(b: &mut Bencher) {
    bench_vec!(b, 1000);
}

#[bench]
fn bench_fxmap_10(b: &mut Bencher) {
    bench_map!(b, 10, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_50(b: &mut Bencher) {
    bench_map!(b, 50, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_250(b: &mut Bencher) {
    bench_map!(b, 250, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_500(b: &mut Bencher) {
    bench_map!(b, 500, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_fxmap_1000(b: &mut Bencher) {
    bench_map!(b, 1000, FxHashMap<u16, u16>, hashmap_vs_vec::fxmap);
}

#[bench]
fn bench_map_10(b: &mut Bencher) {
    bench_map!(b, 10, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_50(b: &mut Bencher) {
    bench_map!(b, 50, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_250(b: &mut Bencher) {
    bench_map!(b, 250, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_500(b: &mut Bencher) {
    bench_map!(b, 500, HashMap<u16, u16>, hashmap_vs_vec::map);
}

#[bench]
fn bench_map_1000(b: &mut Bencher) {
    bench_map!(b, 1000, HashMap<u16, u16>, hashmap_vs_vec::map);
}
