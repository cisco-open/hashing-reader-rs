// Copyright 2023 Cisco Systems, Inc.
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use ::hashing_reader::HashingReader;
#[cfg(feature = "tokio")]
use criterion::async_executor::FuturesExecutor;
use criterion::*;
use sha2::Sha512;
use std::io::Cursor;

#[cfg(feature = "tokio")]
async fn async_plain_run(data: &[u8]) {
    let mut buffer = vec![0_u8; 256];
    let mut cursor = Cursor::new(data);
    tokio::io::AsyncReadExt::read_to_end(&mut cursor, &mut buffer)
        .await
        .unwrap();
}

#[cfg(feature = "tokio")]
async fn async_hasher_run(data: &[u8]) {
    let mut buffer = vec![0_u8; 256];
    let cursor = Cursor::new(data);
    let (mut wrapper, mut _wrapper_hasher) = HashingReader::<_, Sha512>::new(cursor);
    tokio::io::AsyncReadExt::read_to_end(&mut wrapper, &mut buffer)
        .await
        .unwrap();
}

fn std_plain_run(data: &[u8]) {
    let mut buffer = vec![0_u8; 256];
    let mut cursor = Cursor::new(data);
    std::io::Read::read_to_end(&mut cursor, &mut buffer).unwrap();
}

fn std_hasher_run(data: &[u8]) {
    let mut buffer = vec![0_u8; 256];
    let cursor = Cursor::new(data);
    let (mut wrapper, mut _wrapper_hasher) = HashingReader::<_, Sha512>::new(cursor);
    std::io::Read::read_to_end(&mut wrapper, &mut buffer).unwrap();
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashing reader vs plain");

    let data = (0..(1024 * 1024)).map(|_| 0).collect::<Vec<u8>>();

    group.bench_function("std plain", |b| b.iter(|| std_plain_run(&data)));

    group.bench_function("std hashing", |b| b.iter(|| std_hasher_run(&data)));

    #[cfg(feature = "tokio")]
    group.bench_function("async plain", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| async { async_plain_run(&data).await })
    });

    #[cfg(feature = "tokio")]
    group.bench_function("async hashing", |b| {
        b.to_async(FuturesExecutor)
            .iter(|| async { async_hasher_run(&data).await })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
