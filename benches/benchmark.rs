use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use imgpx::{config::Config, render};

fn criterion_benchmark(c: &mut Criterion) {
    let base_cfg = Config {
        input_file: "./the_starry_night.jpg".to_string(),
        sprite_set_path: "minecraft-old".to_string(),
        output_dest: "./out.png".to_string(),
        pixel_size: 16,
        img_pixel_width: 128,
        disable_caching: false,
    };

    let mut default_group = c.benchmark_group("default");

    default_group.bench_function("the_starry_night", |b| {
        b.iter(|| render(black_box(&base_cfg)))
    });

    default_group.bench_function("mona_lisa", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./mona_lisa.jpg".to_string();

        b.iter(|| render(black_box(&cfg)))
    });

    default_group.bench_function("vibrant_flowers", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./vibrant_flowers.jpg".to_string();
        b.iter(|| render(black_box(&cfg)))
    });

    default_group.finish();

    let mut small_group = c.benchmark_group("small");

    small_group.bench_function("the_starry_night", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./the_starry_night.jpg".to_string();
        cfg.img_pixel_width = 64;

        b.iter(|| render(black_box(&cfg)))
    });

    small_group.bench_function("mona_lisa", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./mona_lisa.jpg".to_string();
        cfg.img_pixel_width = 64;

        b.iter(|| render(black_box(&cfg)))
    });

    small_group.bench_function("vibrant_flowers", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./vibrant_flowers.jpg".to_string();
        cfg.img_pixel_width = 64;

        b.iter(|| render(black_box(&cfg)))
    });

    small_group.finish();

    let mut large_group = c.benchmark_group("large");

    large_group.bench_function("the_starry_night", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./the_starry_night.jpg".to_string();
        cfg.img_pixel_width = 256;

        b.iter(|| render(black_box(&cfg)))
    });

    large_group.bench_function("mona_lisa", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./mona_lisa.jpg".to_string();
        cfg.img_pixel_width = 256;

        b.iter(|| render(black_box(&cfg)))
    });

    large_group.bench_function("vibrant_flowers", |b| {
        let mut cfg = base_cfg.clone();
        cfg.input_file = "./vibrant_flowers.jpg".to_string();
        cfg.img_pixel_width = 256;

        b.iter(|| render(black_box(&cfg)))
    });

    large_group.finish();
}

criterion_group!(
   name = benches;
    config = Criterion::default().sample_size(100).measurement_time(Duration::from_secs(20));
    targets = criterion_benchmark
);
criterion_main!(benches);
