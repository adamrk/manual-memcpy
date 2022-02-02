use criterion::{criterion_group, criterion_main, Criterion};

const KB: usize = 1024;
const MB: usize = 1024 * 1024;

fn manual_memcpy(dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len());
    for i in 0..dst.len() {
        dst[i] = src[i];
    }
}

fn create_aligned_vecs(size: usize) -> (Vec<u8>, Vec<u8>) {
    let l = vec![0u8; size];
    let r = vec![1u8; size];
    assert_eq!(l.as_ptr() as usize % 8, 0);
    assert_eq!(r.as_ptr() as usize % 8, 0);
    (l, r)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("copy");
    let sizes = [
        // ("16B", 16),
        // ("128B", 128),
        // ("1KiB", KB),
        ("16KiB", 16 * KB),
        ("128KiB", 128 * KB),
        ("1MiB", MB),
        ("16MiB", 16 * MB),
    ];
    for (name, size) in sizes {
        group.bench_function(format!("{}_manual", name), |b| {
            b.iter_batched_ref(
                || create_aligned_vecs(size),
                |(dst, src)| manual_memcpy(dst, src),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_function(format!("{}_stdlib", name), |b| {
            b.iter_batched_ref(
                || create_aligned_vecs(size),
                |(dst, src)| dst.copy_from_slice(src),
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
