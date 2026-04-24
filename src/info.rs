use sysinfo::{System, Disks, Networks};

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
            boot: System::boot_time().to_string(),
            uptime: System::uptime().to_string(),
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
                usage: cpu.cpu_usage().to_string(),
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
        VnMemory { 
            total: sys.total_memory().to_string(), 
            available: sys.available_memory().to_string(), 
            used: sys.used_memory().to_string(), 
            free: sys.free_memory().to_string(), 
            total_swap: sys.total_swap().to_string(), 
            used_swap: sys.used_swap().to_string(), 
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
                total_space: d.total_space().to_string(),
                available_space: d.available_space().to_string(),
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
                received: n.1.received().to_string(),
                transmitted: n.1.transmitted().to_string(),
                total_received: n.1.total_received().to_string(),
                total_transmitted: n.1.total_transmitted().to_string(),
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
    uid: String,
    pid: String,
    cpu_usage: String,
    memory_usage: String,
    start_time: String,
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
                name: process.name().to_str().unwrap_or("Unknown Namme").to_string(),
                uid: process.user_id()
                    .map(|u| u.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
                pid: pid.as_u32().to_string(),
                cpu_usage: process.cpu_usage().to_string(),
                memory_usage: process.memory().to_string(),
                start_time: process.start_time().to_string(),
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

            lines.push(format!("{:<8} | {:<8} | {:<25} | {:<8} | {:<10} | {:<10} | {:<8}", 
                p.pid, p.uid, short_name, cpu_str, p.memory_usage, p.status, p.start_time));
        }

        lines.join("\n")
    }
}