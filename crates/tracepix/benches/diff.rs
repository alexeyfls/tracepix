use std::{hint::black_box, path::Path, time::Duration};

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use tracepix::{CompareOptions, Image, compare_images};

const REFERENCE: &str = "images/water-4k.png";
const TARGET: &str = "images/water-4k-2.png";

fn options(detect_antialiasing: bool) -> CompareOptions {
    CompareOptions {
        threshold: 0.1,
        detect_antialiasing,
        emit_diff_image: false,
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let (reference, target) = Image::load_pair(Path::new(REFERENCE), Path::new(TARGET))
        .expect("bench fixtures should load");
    let pixels = reference.width as u64 * reference.height as u64;

    let mut group = c.benchmark_group("pipeline");
    group.throughput(Throughput::Elements(pixels));

    group.bench_function("decode", |b| {
        b.iter(|| {
            Image::load_pair(
                black_box(Path::new(REFERENCE)),
                black_box(Path::new(TARGET)),
            )
            .expect("bench fixtures should load")
        });
    });

    group.bench_function("diff", |b| {
        b.iter(|| compare_images(black_box(&reference), black_box(&target), &options(false)));
    });

    group.bench_function("diff_aa", |b| {
        b.iter(|| compare_images(black_box(&reference), black_box(&target), &options(true)));
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
