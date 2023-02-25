use std::io::Write;

use sysinfo::{Process, ProcessExt, System, SystemExt};

fn main() {
    let s = System::new_all();

    let mut processes: Vec<&Process> = s.processes().values().collect();
    processes.sort_by(|x, y| x.memory().partial_cmp(&y.memory()).unwrap());

    let mut stdout = std::io::stdout().lock();
    let mut total = 0;
    for process in processes {
        total += process.memory();
        stdout
            .write_fmt(format_args!(
                "{}MB\t[{:<10}]\t{}\n",
                process.memory() / 1024 / 1024,
                process.pid(),
                process.name()
            ))
            .unwrap();
    }

    stdout.write_all(b"\n").unwrap();
    stdout
        .write_fmt(format_args!(
            "Total: {:.3} GB\n",
            total as f64 / 1024f64 / 1024f64 / 1024f64
        ))
        .unwrap();
}
