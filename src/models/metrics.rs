// use axum::Json;
use sysinfo::{System, SystemExt, DiskExt};
use serde::Serialize;
// use serde_json::json;

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
struct DiskInfo {
    mount_point: String,
    spaces: Spaces,
}
#[derive(Serialize)]
struct Spaces {
    available_space: u64,
    total_space: u64,
}

pub async fn get_storage() -> Vec<String> {
    let sys = System::new_all();
    let mut disk_info = Vec::new();

    for disk in sys.disks() {
        let diskinfo = DiskInfo {
            mount_point: disk.mount_point().to_path_buf().to_string_lossy().into_owned(),
            spaces: Spaces {
                available_space: disk.available_space(),
                total_space: disk.total_space(),
            }
        };
        let serialized = serde_json::to_string(&diskinfo).unwrap();
        disk_info.push(serialized);
    }

    // case return Json string
    // Json(json!(disk_info)).to_string()

    // case return string
    // let string = disk_info.join(",");
    // string

    // case return Vec<String>
    disk_info
}
