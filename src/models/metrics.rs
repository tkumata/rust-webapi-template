use axum::Json;
use sysinfo::{System, SystemExt, DiskExt};
use serde::Serialize;
use serde_json::json;

pub async fn get_kernelname() -> Option<String> {
    let sys = System::new();

    sys.name()
}

pub async fn get_load() -> String {
    let sys = System::new_all();
    let load_avg = sys.load_average();

    [
        load_avg.one.to_string(),
        load_avg.five.to_string(),
        load_avg.fifteen.to_string()
    ].join(",")
}

pub async fn get_mem() -> u64 {
    let sys = System::new_all();

    sys.used_memory() as u64
}

#[derive(Serialize)]
struct ApiDiskInfo {
    mount_point: String,
    available_space: u64,
    total_space: u64,
}

pub async fn get_storage() -> String {
    let sys = System::new_all();
    let mut disk_info = Vec::new();

    for disk in sys.disks() {
        disk_info.push(ApiDiskInfo {
            mount_point: disk.mount_point().to_path_buf().to_string_lossy().into_owned(),
            available_space: disk.available_space(),
            total_space: disk.total_space(),
        });
    }

    Json(json!(disk_info)).to_string()
}
