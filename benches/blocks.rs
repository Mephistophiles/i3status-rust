extern crate i3status_rs;

use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam_channel::{Receiver, Sender};
use i3status_rs::{
    blocks::{
        cpu::{Cpu, CpuConfig},
        load::{Load, LoadConfig},
        memory::{Memory, MemoryConfig},
        time::{Time, TimeConfig},
        Block, ConfigBlock,
    },
    config::SharedConfig,
    scheduler::Task,
};

macro_rules! get_block {
    ($module_name:ident, $config_name:ident) => {{
        let (tx_update_requests, _rx_update_requests): (Sender<Task>, Receiver<Task>) =
            crossbeam_channel::unbounded();
        let shared_config = SharedConfig::default();
        let config = $config_name::default();
        $module_name::new(1, config, shared_config, tx_update_requests).unwrap()
    }};
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut load = get_block!(Load, LoadConfig);
    let mut cpu = get_block!(Cpu, CpuConfig);
    let mut memory = get_block!(Memory, MemoryConfig);
    let mut time = get_block!(Time, TimeConfig);

    c.bench_function("load", |b| b.iter(|| load.update()));
    c.bench_function("cpu", |b| b.iter(|| cpu.update()));
    c.bench_function("memory", |b| b.iter(|| memory.update()));
    c.bench_function("time", |b| b.iter(|| time.update()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
