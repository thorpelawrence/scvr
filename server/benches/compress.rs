use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use utils::compress::{CompressionFormat, CompressionLevel};

mod inputs;

pub fn compress_levels(c: &mut Criterion) {
    let bytes = inputs::get_image(inputs::SCREENSHOT).into_bytes();

    let mut group = c.benchmark_group("Compression levels");
    group.throughput(Throughput::Elements(1));

    for (sample_size, level) in [
        (100, CompressionLevel::Fast),
        (30, CompressionLevel::Default),
    ]
    .iter()
    {
        group.sample_size(*sample_size);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", level)),
            level,
            |b, &level| b.iter(|| utils::compress::compress(&bytes, Some(level), None)),
        );
    }
    group.finish();
}

pub fn compress_formats(c: &mut Criterion) {
    let bytes = inputs::get_image(inputs::SCREENSHOT).into_bytes();

    let mut group = c.benchmark_group("Compression formats");
    group.throughput(Throughput::Elements(1));

    for (sample_size, format) in [
        (100, CompressionFormat::Deflate),
        (100, CompressionFormat::Gzip),
        (100, CompressionFormat::None),
    ]
    .iter()
    {
        group.sample_size(*sample_size);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", format)),
            format,
            |b, &level| b.iter(|| utils::compress::compress(&bytes, None, Some(level))),
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = compress_levels, compress_formats
}
criterion_main!(benches);
