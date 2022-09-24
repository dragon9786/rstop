#![deny(clippy::all)]

extern crate colorful;
mod args;
mod components;

use crate::components::components::Process;
use args::RsTopArgs;
use clap::Parser;
use colorful::{Color, Colorful};
use itertools::Itertools;
use std::cmp::Reverse;
use sysinfo::{ProcessExt, System, SystemExt};
use tabled::{object::Columns, Format, Modify, Style, Table};
use termion::{clear, cursor};
use tokio::time::{sleep, Duration};

const GB_IN_BYTES: u64 = 1000000;
#[tokio::main]
async fn main() {
    let args = RsTopArgs::parse();
    loop {
        let mut sys = System::new_all();
        print!("{}{}", clear::All, cursor::Goto(1, 1));

        sys.refresh_processes();

        let mut processes = Vec::new();

        let sorted_processes = sys
            .processes()
            .iter()
            .sorted_by_key(|x| Reverse(x.1.memory()));

        for (pid, process) in sorted_processes {
            let mem = process.memory().checked_div(GB_IN_BYTES).unwrap();
            let virt_mem = process.memory().checked_div(GB_IN_BYTES).unwrap();
            let process_to_table = Process {
                pid: pid.to_string(),
                command: process.name(),
                mem: mem as f32,
                user: process.name(),
                cpu: process.cpu_usage() as f32,
                virt_mem: virt_mem as f32,
            };
            processes.push(process_to_table);
        }
        let table = Table::new(&processes)
            .with(Style::rounded())
            .with(
                Modify::new(Columns::single(0))
                    .with(Format::new(|s| s.gradient(Color::Red).to_string())),
            )
            .to_string();
        processes.clear();
        println!("{}", table);
        sleep(Duration::from_secs((args.interval - 1.0) as u64)).await;
    }
}
