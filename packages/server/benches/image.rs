use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use inputs::get_image;
use utils::image::{bgra_to_image, vr_transform, Dimensions};

mod inputs;

fn buffer(c: &mut Criterion) {
    let img = get_image(inputs::SCREENSHOT).into_bgra8();
    let (w, h) = (img.width() as usize, img.height() as usize);
    let bytes = img.into_raw();

    let mut group = c.benchmark_group("BGRA conversion");
    group.throughput(Throughput::Elements(1));

    group.bench_with_input(
        BenchmarkId::from_parameter(inputs::SCREENSHOT),
        &bytes,
        |b, bytes| {
            b.iter(|| {
                bgra_to_image(
                    &bytes,
                    Dimensions {
                        width: w,
                        height: h,
                    },
                )
            })
        },
    );
    group.finish();
}

pub fn vr(c: &mut Criterion) {
    use utils::image::ResizeAlgorithm;
    let img = get_image(inputs::SCREENSHOT);

    let mut group = c.benchmark_group("Resizing algorithms");
    group.throughput(Throughput::Elements(1));

    for (sample_size, alg) in [
        (50, ResizeAlgorithm::NearestNeighbour),
        (30, ResizeAlgorithm::Linear),
        (20, ResizeAlgorithm::Cubic),
        (20, ResizeAlgorithm::Gaussian),
        (20, ResizeAlgorithm::Lanczos3),
    ]
    .iter()
    {
        group.sample_size(*sample_size);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", alg)),
            alg,
            |b, &alg| {
                b.iter(|| {
                    vr_transform(
                        &img,
                        Dimensions {
                            width: 1920,
                            height: 1080,
                        },
                        60,
                        1.15,
                        alg,
                        false,
                    )
                })
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = buffer, vr
}
criterion_main!(benches);
