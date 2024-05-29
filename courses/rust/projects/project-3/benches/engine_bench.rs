use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use tempfile::TempDir;

use kvs::{KvsEngine, KvStore, SledKvsEngine};

/// Criterion
///     The benchmark manager
///     Criterion lets you configure and execute benchmarks
///     Each benchmark consists of four phases
///         - warm-up： The routine is repeatedly executed， to let the cup/os/jit/interpreter adapt to the new load
///         - measurement: The routine is repeatedly executed, and timing information is collected into a sample
///         - analysis: the sample is analyzed and distilled into meaningful statistics that get reported to stdout, sorted in files, and plotted
///         - comparison: the current sample is compared with the sample obtained in the previous benchmark.
fn set_bench(c: &mut Criterion) {
    // Return a benchmark group. All benchmarks performed using a benchmark group will be grouped together in the final report
    let mut group = c.benchmark_group("set_bench");

    // Benchmark the given parameterless function this benchmark group
    group.bench_function("kvs", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                (KvStore::open(temp_dir.path()).unwrap(), temp_dir)
            },
            |(mut store, _temp_dir)| {
                for i in 1..(1 << 12) {
                    store.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("sled", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                (SledKvsEngine::new(sled::open(&temp_dir).unwrap()), temp_dir)
            },
            |(mut store, _temp_dir)| {
                for i in 1..(1 << 12) {
                    store.set(format!("key{}", i), "value".to_string()).unwrap();
                }
            },
            BatchSize::SmallInput,
        )
    });}

fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_bench");

    for i in &vec![8, 12, 16, 20] {
        group.bench_with_input(format!("kvs_{}", i), i, |b, i| {
            let temp_dir = TempDir::new().unwrap();
            let mut store = KvStore::open(temp_dir.path()).unwrap();
            for key_i in 1..(1 << i) {
                store
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }

            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                store
                    .get(format!("key{}", rng.gen_range(1, 1 << i)))
            })
        });


    }

    for i in &vec![8, 12, 16, 20] {
        group.bench_with_input(format!("sled_{}", i), i, |b, i| {
            let temp_dir = TempDir::new().unwrap();
            let mut db = SledKvsEngine::new(sled::open(&temp_dir).unwrap());
            for key_i in 1..(1 << i) {
                db
                    .set(format!("key{}", key_i), "value".to_string())
                    .unwrap();
            }

            let mut rng = SmallRng::from_seed([0; 16]);
            b.iter(|| {
                db
                    .get(format!("key{}", rng.gen_range(1, 1 << i)))
            })
        });


    }
}

criterion_group!(benches, set_bench, get_bench);
criterion_main!(benches);