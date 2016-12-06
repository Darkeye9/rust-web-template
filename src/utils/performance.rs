extern crate chrono;

use rustc_serialize::json::{ToJson, Json};

use std::sync::RwLock;
use std::{thread, time};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::BTreeMap;

lazy_static! {
    static ref PERF_COLLECTOR: RwLock<PerfomanceCollector> = RwLock::new(PerfomanceCollector::default());
    static ref PID_MATCHER: Regex = Regex::new(r"Pid:\s+(\d+)").unwrap();
    static ref MEM_RES_MATCHER: Regex = Regex::new(r"VmRSS:\s+(\d+)").unwrap();
    static ref VIRTUAL_MEM_MATCHER: Regex = Regex::new(r"VmSize:\s+(\d+)").unwrap();
    static ref THREADS_MATCHER: Regex = Regex::new(r"Threads:\s+(\d+)").unwrap();
}

#[derive(Default, Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct MemStats {
    resident_mem: u32,
    resident_mem_max: u32,
    virtual_mem: u32,
    virtual_mem_max: u32,
    threads: u32,
    threads_max: u32,
    pid: u32,
}

impl ToJson for MemStats {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("resident_mem".to_string(),
                 format!("{:.3}", self.resident_mem as f32 / 1000.0).to_json());
        m.insert("resident_mem_max".to_string(),
                 format!("{:.3}", self.resident_mem_max as f32 / 1000.0).to_json());
        m.insert("virtual_mem".to_string(),
                 format!("{:.3}", self.virtual_mem as f32 / 1000.0).to_json());
        m.insert("virtual_mem_max".to_string(),
                 format!("{:.3}", self.virtual_mem_max as f32 / 1000.0).to_json());
        m.insert("threads".to_string(), self.threads.to_json());
        m.insert("threads_max".to_string(), self.threads_max.to_json());
        m.insert("pid".to_string(), self.pid.to_json());
        // println!("{:?}\n{:?}", self, m);
        m.to_json()
    }
}

#[derive(Default, Debug, Clone)]
struct StatusResult {
    resident_mem: u32,
    virtual_mem: u32,
    threads: u32,
    pid: u32,
}

#[derive(Default, Debug, Clone)]
pub struct PerfomanceCollector {
    mem_stats: MemStats,
}

impl PerfomanceCollector {
    pub fn init() {
        thread::spawn(move || {
            // Sleep an initial amount of time, to let the app initialice completely.
            thread::sleep(time::Duration::new(5, 0));

            println!("Perfomance collectors initialized");
            loop {
                update_perf_collect();
                thread::sleep(time::Duration::new(60, 0));
            }
        });
    }

    #[allow(dead_code)]
    pub fn get_stats() -> Option<MemStats> {
        match PERF_COLLECTOR.read() {
            Ok(perf_collector) => Some(perf_collector.mem_stats.clone()),
            Err(e) => {
                println!("Error getting a read lock on PerfomanceCollector: {:?}", e);
                None
            }
        }
    }
}

fn update_perf_collect() {
    //println!("Im updating!");
    if let Some(new_stats) = read_proc_status() {
        match PERF_COLLECTOR.write() {
            Ok(mut perf_collector) => {
                perf_collector.mem_stats.pid = new_stats.pid;
                perf_collector.mem_stats.resident_mem = new_stats.resident_mem;
                perf_collector.mem_stats.virtual_mem = new_stats.virtual_mem;
                perf_collector.mem_stats.threads = new_stats.threads;

                if perf_collector.mem_stats.resident_mem_max <
                   perf_collector.mem_stats.resident_mem {
                    perf_collector.mem_stats.resident_mem_max = perf_collector.mem_stats
                        .resident_mem;
                }
                if perf_collector.mem_stats.virtual_mem_max < perf_collector.mem_stats.virtual_mem {
                    perf_collector.mem_stats.virtual_mem_max = perf_collector.mem_stats.virtual_mem;
                }
                if perf_collector.mem_stats.threads_max < perf_collector.mem_stats.threads {
                    perf_collector.mem_stats.threads_max = perf_collector.mem_stats.threads;
                }
            }
            Err(e) => {
                println!("Error getting a write lock on PerfomanceCollector: {:?}", e);
            }
        }
        // println!("{:?}", new_stats);
    }
}

fn read_proc_status() -> Option<StatusResult> {
    match File::open("/proc/self/status") {
        Ok(mut file) => {
            let mut content = String::new();
            let _ = file.read_to_string(&mut content);
            let mut results = StatusResult::default();
            // println!("{}", content);

            if let Some(matches) = PID_MATCHER.captures(content.as_str()) {
                if let Some(pid) = matches.at(1) {
                    results.pid = pid.parse::<u32>().unwrap();
                }
            }
            if let Some(matches) = MEM_RES_MATCHER.captures(content.as_str()) {
                if let Some(mem_res) = matches.at(1) {
                    results.resident_mem = mem_res.parse::<u32>().unwrap();
                }
            }
            if let Some(matches) = VIRTUAL_MEM_MATCHER.captures(content.as_str()) {
                if let Some(virt_mem) = matches.at(1) {
                    results.virtual_mem = virt_mem.parse::<u32>().unwrap();
                }
            }
            if let Some(matches) = THREADS_MATCHER.captures(content.as_str()) {
                if let Some(threads) = matches.at(1) {
                    results.threads = threads.parse::<u32>().unwrap();
                }
            }
            Some(results)
        }
        Err(e) => {
            println!("Error accessing /proc/self/status: {:?}", e);
            None
        }
    }
}
