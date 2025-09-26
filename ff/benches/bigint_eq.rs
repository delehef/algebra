use std::hint::black_box;

use ark_ff::{BigInt, UniformRand};
use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const NUM_TRIES: usize = 10_000;

fn cmp_many<const N: usize>(xs: &[BigInt<N>], ys: &[BigInt<N>]) {
    black_box(
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| x == y)
            .collect::<Vec<_>>(),
    );
}

fn bench_eq(c: &mut Criterion) {
    let mut group = c.benchmark_group("BigInt::Eq");
    let mut rng = test_rng();

    group.bench_with_input(BenchmarkId::from_parameter(2), &2, |b, _width| {
        let xs: Vec<BigInt<2>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<2>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.bench_with_input(BenchmarkId::from_parameter(5), &5, |b, _width| {
        let xs: Vec<BigInt<5>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<5>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.bench_with_input(BenchmarkId::from_parameter(6), &6, |b, _width| {
        let xs: Vec<BigInt<6>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<6>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.bench_with_input(BenchmarkId::from_parameter(7), &7, |b, _width| {
        let xs: Vec<BigInt<7>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<7>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.bench_with_input(BenchmarkId::from_parameter(8), &8, |b, _width| {
        let xs: Vec<BigInt<8>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<8>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.bench_with_input(BenchmarkId::from_parameter(12), &12, |b, _width| {
        let xs: Vec<BigInt<12>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        let ys: Vec<BigInt<12>> = (0..NUM_TRIES).map(|_| BigInt::rand(&mut rng)).collect();
        b.iter(|| black_box(cmp_many(&xs, &ys)));
    });
    group.finish();
}

criterion_group!(benches, bench_eq);
criterion_main!(benches);
