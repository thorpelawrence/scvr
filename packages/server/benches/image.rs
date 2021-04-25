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
    use image::imageops::FilterType;
    let img = get_image(inputs::SCREENSHOT);

    let mut group = c.benchmark_group("Resizing algorithms");
    group.throughput(Throughput::Elements(1));

    for (sample_size, alg) in [
        (50, FilterType::Nearest),
        (30, FilterType::Triangle),
        (20, FilterType::CatmullRom),
        (20, FilterType::Gaussian),
        (20, FilterType::Lanczos3),
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
                        Some(alg),
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
