use sysinfo::{System, SystemExt};

pub async fn get_load() -> String {
    let sys = System::new_all();
    let load_avg = sys.load_average();
    let s = [load_avg.one.to_string(), load_avg.five.to_string(), load_avg.fifteen.to_string()].join(",");
    s
}

pub async fn get_mem() -> u64 {
    let sys = System::new_all();
    let mem_usage = sys.used_memory() as u64;
    mem_usage
}
