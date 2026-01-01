// 若数据量字段中未注明单位，则以字节 (Bytes) 为单位
// 若速度字段中未注明单位，则以字节每秒 (Bytes per second) 为单位

struct MonitoringData {
    cpu: CPUData,
    ram: RamData,
    load: LoadData,
    system: SystemData,
    disk: Vec<PerDiskData>,
    network: Vec<PerNetworkInterfaceData>,
}

struct CPUData {
    per_core: Vec<PerCpuCoreData>,
    physical_cores: u64,
    logical_cores: u64,
    total_cpu_usage: f64,
}

struct PerCpuCoreData {
    name: String,
    cpu_usage: f64,
    frequency_mhz: u64,
    vendor_id: String,
    brand: String,
}

struct RamData {
    total_memory: u64,
    available_memory: u64,
    used_memory: u64,

    total_swap: u64,
    available_swap: u64,
    used_swap: u64,
}

struct LoadData {
    one: f64,
    five: f64,
    fifteen: f64,
}

struct SystemData {
    system_name: String,
    system_kernel: String,
    system_kernel_version: String,
    system_os_version: String,
    system_os_long_version: String,
    distribution_id: String,
    system_host_name: String,
    arch: String,
    boot_time: u64,
    uptime: u64,
    process_count: u64,
}

struct PerDiskData {
    kind: String, // e.g., SSD, HDD
    name: String,
    file_system: String,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
    is_read_only: bool,
    read_speed: u64,
    write_speed: u64,
}

struct PerNetworkInterfaceData {
    interface_name: String,
    total_received: u64,
    total_transmitted: u64,
    receive_speed: u64,
    transmit_speed: u64,
}
