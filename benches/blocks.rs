extern crate i3status_rs;

use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam_channel::{Receiver, Sender};
use i3status_rs::{
    blocks::{
        load::{Load, LoadConfig},
        Block, ConfigBlock,
    },
    config::SharedConfig,
    scheduler::Task,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let (tx_update_requests, _rx_update_requests): (Sender<Task>, Receiver<Task>) =
        crossbeam_channel::unbounded();
    let shared_config = SharedConfig::default();
    let load_config = LoadConfig::default();
    let mut load = Load::new(1, load_config, shared_config, tx_update_requests).unwrap();
    c.bench_function("load", |b| b.iter(|| load.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
