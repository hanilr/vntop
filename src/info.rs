use sysinfo::{System, Disks, Networks};

fn format_bytes(bytes: u64) -> String {
    let bytes_f = bytes as f64;
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;
    let tb = gb * 1024.0;

    if bytes_f >= tb {
        format!("{:.2} TB", bytes_f / tb)
    } else if bytes_f >= gb {
        format!("{:.2} GB", bytes_f / gb)
    } else if bytes_f >= mb {
        format!("{:.2} MB", bytes_f / mb)
    } else if bytes_f >= kb {
        format!("{:.2} KB", bytes_f / kb)
    } else {
        format!("{} B", bytes)
    }
}

fn format_time(seconds: u64) -> String {
    if seconds == 0 { return "None".to_string(); }

    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds_rem = seconds % 60;

    if days != 0 {
        format!("{}d, {}h, {}m, {}s", days, hours, minutes, seconds_rem)
    } else if hours != 0 {
        format!("{}h, {}m, {}s", hours, minutes, seconds_rem)
    } else if minutes != 0 {
        format!("{}m, {}s", minutes, seconds_rem)
    } else {
        format!("{}s", seconds_rem)
    }
}

// System informations.
pub struct VnSystem {
    name: String,
    host: String,
    boot: String,
    uptime: String,
    arch: String,
    os: String,
    long_os: String,
    kernel: String,
    long_kernel: String,
}

impl VnSystem {
    pub fn new() -> VnSystem {
        VnSystem { 
            name: System::name().unwrap_or_default(),
            host: System::host_name().unwrap_or_default(),
            boot: format_time(System::boot_time()),
            uptime: format_time(System::uptime()),
            arch: System::cpu_arch(),
            os: System::os_version().unwrap_or_default(),
            long_os: System::long_os_version().unwrap_or_default(),
            kernel: System::kernel_version().unwrap_or_default(),
            long_kernel: System::kernel_long_version(),
        }
    }

    pub fn refresh(&mut self) {
        let new = VnSystem::new();
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", self.name,
            self.host, self.boot, self.uptime, self.arch,
            self.os, self.long_os, self.kernel, self.long_kernel) as String
    }
}

// Only Contain a cores informations.
struct VnCpuCore {
    name: String,
    usage: String,
    freq: String,
}

// Contain general and every cores informations.
pub struct VnCpu {
    brand: String,
    vendor: String,
    logical: String,
    physical: String,
    total_usage: String,
    core: Vec<VnCpuCore>,
}

impl VnCpu {
    pub fn new(sys: &System) -> VnCpu {
        let mut cpu_core = vec![];
        for cpu in sys.cpus() {
            let info = VnCpuCore {
                name: cpu.name().to_string(),
                usage: format!("%{:.1}", cpu.cpu_usage()),
                freq: cpu.frequency().to_string(),
            };
            cpu_core.push(info);
        }

        VnCpu { 
            brand: sys.cpus()[0].brand().to_string(), 
            vendor: sys.cpus()[0].vendor_id().to_string(), 
            logical: sys.cpus().len().to_string(), 
            physical: System::physical_core_count().unwrap_or(0).to_string(), 
            total_usage: sys.global_cpu_usage().to_string(), 
            core: cpu_core,
        }
    }

    pub fn refresh(&mut self, sys: &System) {
        let new = VnCpu::new(sys);
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        if self.core.is_empty() {
            return "N/A\nN/A\nN/A\nN/A\nN/A\nN/A\nN/A\nN/A\nN/A".to_string();
        }

        let core_names = self.core.iter().map(|c| c.name.clone()).collect::<Vec<String>>().join(", ");
        let core_usages = self.core.iter().map(|c| c.usage.clone()).collect::<Vec<String>>().join(", ");
        let core_freqs = self.core.iter().map(|c| c.freq.clone()).collect::<Vec<String>>().join(", ");
        let code = "N/A".to_string();

        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", 
            core_names, 
            core_usages, 
            core_freqs, 
            self.brand, 
            self.vendor, 
            self.logical, 
            self.physical, 
            self.total_usage, 
            code
        )
    }
}

// Memory informations.
pub struct VnMemory {
    total: String,
    available: String,
    used: String,
    free: String,
    total_swap: String,
    used_swap: String,
}

impl VnMemory {
    pub fn new(sys: &System) -> VnMemory {
        let total_mem = format_bytes(sys.total_memory());
        let available_mem = format_bytes(sys.available_memory());
        let used = format_bytes(sys.used_memory());
        let free = format_bytes(sys.free_memory());
        let total_swap = format_bytes(sys.total_swap());
        let used_swap = format_bytes(sys.used_swap());

        VnMemory { 
            total: total_mem, 
            available: available_mem, 
            used: used, 
            free: free, 
            total_swap: total_swap, 
            used_swap: used_swap, 
        }
    }

    pub fn refresh(&mut self, sys: &System) {
        let new = VnMemory::new(sys);
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}", self.total,
            self.available, self.used, self.free, self.total_swap,
            self.used_swap) as String
    }
}

// Contain informations of per disk.
struct VnDiskInfo {
    name: String,
    mount: String,
    kind: String,
    file_type: String,
    total_space: String,
    available_space: String,
}

// Contain all disks informations.
pub struct VnDisk {
    disk_info: Vec<VnDiskInfo>,
}

impl VnDisk {
    pub fn new(disks: &Disks) -> VnDisk {
        let mut disk_info = vec![];

        for d in disks.list() {
            let info = VnDiskInfo {
                name: d.name().to_str().unwrap_or("Unknown Name").to_string(),
                mount: d.mount_point().to_str().unwrap_or("Unknown Mount").to_string(),
                kind: d.kind().to_string(),
                file_type: d.file_system().to_str().unwrap_or("Unknown File System").to_string(),
                total_space: format_bytes(d.total_space()),
                available_space: format_bytes(d.available_space()),
            };
            disk_info.push(info);
        }

        VnDisk { 
            disk_info: disk_info, 
        }
    }

    pub fn refresh(&mut self, disks: &Disks) {
        let new = VnDisk::new(disks);
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        if self.disk_info.is_empty() {
            return "N/A\nN/A\nN/A\nN/A\nN/A\nN/A".to_string();
        }

        let names = self.disk_info.iter().map(|d| d.name.clone()).collect::<Vec<String>>().join(", ");
        let mounts = self.disk_info.iter().map(|d| d.mount.clone()).collect::<Vec<String>>().join(", ");
        let kinds = self.disk_info.iter().map(|d| d.kind.clone()).collect::<Vec<String>>().join(", ");
        let file_types = self.disk_info.iter().map(|d| d.file_type.clone()).collect::<Vec<String>>().join(", ");
        let totals = self.disk_info.iter().map(|d| d.total_space.clone()).collect::<Vec<String>>().join(", ");
        let availables = self.disk_info.iter().map(|d| d.available_space.clone()).collect::<Vec<String>>().join(", ");

        format!("{}\n{}\n{}\n{}\n{}\n{}", names, mounts, kinds, file_types, totals, availables)
    }
}

// Contains informations of per network devices.
struct VnNetworkInfo {
    name: String,
    mac: String,
    received: String,
    transmitted: String,
    total_received: String,
    total_transmitted: String,
}

// Contains every network devices informations.
pub struct VnNetwork {
    network_info: Vec<VnNetworkInfo>,
}

impl VnNetwork {
    pub fn new(networks: &Networks) -> VnNetwork {
        let mut network_info = vec![];

        for n in networks.list() {
            let info = VnNetworkInfo {
                name: n.0.to_string(),
                mac: n.1.mac_address().to_string(),
                received: format_bytes(n.1.received()),
                transmitted: format_bytes(n.1.transmitted()),
                total_received: format_bytes(n.1.total_received()),
                total_transmitted: format_bytes(n.1.total_transmitted()),
            };
            network_info.push(info);
        }

        VnNetwork {
            network_info: network_info,
        }
    }

    pub fn refresh(&mut self, networks: &Networks) {
        let new = VnNetwork::new(networks);
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        if self.network_info.is_empty() {
            return "N/A\nN/A\nN/A\nN/A\nN/A\nN/A".to_string();
        }

        let names = self.network_info.iter().map(|n| n.name.clone()).collect::<Vec<String>>().join(", ");
        let macs = self.network_info.iter().map(|n| n.mac.clone()).collect::<Vec<String>>().join(", ");
        let recvs = self.network_info.iter().map(|n| n.received.clone()).collect::<Vec<String>>().join(", ");
        let trans = self.network_info.iter().map(|n| n.transmitted.clone()).collect::<Vec<String>>().join(", ");
        let tot_recvs = self.network_info.iter().map(|n| n.total_received.clone()).collect::<Vec<String>>().join(", ");
        let tot_trans = self.network_info.iter().map(|n| n.total_transmitted.clone()).collect::<Vec<String>>().join(", ");

        format!("{}\n{}\n{}\n{}\n{}\n{}", names, macs, recvs, trans, tot_recvs, tot_trans)
    }
}

// Contains a process informations.
struct VnProcessInfo {
    name: String,
    pid: String,
    cpu_usage: String,
    memory_usage: String,
    status: String,
}

// Contains every processes informations.
pub struct VnProcess {
    process_info: Vec<VnProcessInfo>,
}

impl VnProcess {
    pub fn new(sys: &System) -> VnProcess {
        let mut process_info = vec![];

        for (pid, process) in sys.processes() {
            let info = VnProcessInfo {
                name: process.name().to_str().unwrap_or("Unknown Name").to_string(),
                pid: pid.as_u32().to_string(),
                cpu_usage: format!("%{}", process.cpu_usage()),
                memory_usage: format_bytes(process.memory()),
                status: process.status().to_string(),
            };
            process_info.push(info)
        }

        VnProcess { 
            process_info: process_info,
        }
    }

    pub fn refresh(&mut self, sys: &System) {
        let new = VnProcess::new(sys);
        *self = new;
    }

    pub fn raw_info(&self) -> String {
        if self.process_info.is_empty() {
            return "N/A".to_string();
        }

        let mut lines = vec![
            format!("{:<8} | {:<25} | {:<8} | {:<10} | {:<10}", "PID", "Name", "CPU %", "Memory (B)", "Status"),
            "-".repeat(73)
        ];

        for p in self.process_info.iter().take(100) {
            let short_name = if p.name.chars().count() > 25 {
                format!("{}...", p.name.chars().take(22).collect::<String>())
            } else {
                p.name.clone()
            };

            let cpu_str = if p.cpu_usage.chars().count() > 6 {
                p.cpu_usage.chars().take(6).collect::<String>()
            } else {
                p.cpu_usage.clone()
            };

            lines.push(format!("{:<8} | {:<25} | {:<8} | {:<10} | {:<10}", 
                p.pid, short_name, cpu_str, p.memory_usage, p.status));
        }

        lines.join("\n")
    }
}