use sysinfo::{System, SystemExt, DiskExt};

pub async fn get_kernelname() -> Option<String> {
    let sys = System::new();
    let name = sys.name();

    name
}

pub async fn get_load() -> String {
    let sys = System::new_all();
    let load_avg = sys.load_average();
    let s = [
        load_avg.one.to_string(),
        load_avg.five.to_string(),
        load_avg.fifteen.to_string()
    ].join(",");

    s
}

pub async fn get_mem() -> u64 {
    let sys = System::new_all();
    let m = sys.used_memory() as u64;

    m
}

pub async fn get_storage() -> String {
    let sys = System::new_all();
    let mut free_spaces: Vec<String> = Vec::new();

    for disk in sys.disks() {
        free_spaces.push(disk.available_space().to_string());
    }

    free_spaces.join(",")
}