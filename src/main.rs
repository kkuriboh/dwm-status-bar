use chrono::Local;
use std::{process::Command, thread::sleep, time::Duration};
use sysinfo::{CpuExt, SystemExt};

fn main() {
    loop {
        Command::new("xsetroot")
            .arg("-name")
            .arg(format_str())
            .output()
            .unwrap();
        sleep(Duration::from_millis(1000));
    }
}

fn format_str() -> String {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    let cpu_usage: f32 = {
        let count = &system.cpus().len();
        let sum: f32 = system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect::<Vec<f32>>()
            .iter()
            .sum();
        sum / *count as f32
    };

    format!(
        " ^c#b16286^HOST: {} | ^c#689d6a^CPU: ^c#d79921^{cpu_usage:.2}% | ^c#458588^RAM: {:.2}GB | ^c#cc241d^{}",
        system.host_name().unwrap(),
        system.used_memory() as f32 / 1024. / 1024.,
        Local::now().format("%Y-%m-%d//%H:%M:%S").to_string()
    )
}
