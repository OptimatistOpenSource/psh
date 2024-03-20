mod host;
mod raw;

#[derive(Debug, PartialEq)]
pub struct TlbSize {
    pub count: u32,
    pub unit: u32,
}

#[derive(Debug, PartialEq)]
pub struct AddressSizes {
    pub phy: u8,  // physical bits.
    pub virt: u8, // virtual bits.
}

#[derive(Debug, PartialEq)]
pub struct Arm64CpuInfo {
    pub processor: usize,
    pub bogomips: f32,
    pub features: Vec<String>,
    pub cpu_implementer: u16,
    pub cpu_architecture: u16,
    pub cpu_variant: u16,
    pub cpu_part: u16,
    pub cpu_revision: u16,
    pub address_sizes: AddressSizes,
}

impl Arm64CpuInfo {
    fn new() -> Arm64CpuInfo {
        Arm64CpuInfo {
            processor: 0,
            bogomips: 0.0,
            features: Vec::<String>::new(),
            cpu_implementer: 0,
            cpu_architecture: 0,
            cpu_variant: 0,
            cpu_part: 0,
            cpu_revision: 0,
            address_sizes: AddressSizes { phy: 0, virt: 0 },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct X86_64CpuInfo {
    pub processor: usize,
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: usize,
    pub model: usize,
    pub stepping: usize,
    pub microcode: String,
    pub cpu_mhz: f64,
    pub cache_size: u32,
    pub physical_id: usize,
    pub siblings: usize,
    pub core_id: usize,
    pub cpu_cores: usize,
    pub apicid: usize,
    pub initial_apicid: usize,
    pub fpu: bool,
    pub fpu_exception: bool,
    pub cpuid_level: usize,
    pub wp: bool, // wp stands for ?
    pub flags: Vec<String>,
    pub bugs: Vec<String>,
    pub bogomips: f32,
    pub tlb_size: TlbSize,
    pub clflush_size: u8,
    pub cache_alignment: u8,
    pub address_sizes: AddressSizes,
    pub power_management: Vec<String>, // Add other fields you want to extract
}

impl X86_64CpuInfo {
    fn new() -> X86_64CpuInfo {
        X86_64CpuInfo {
            processor: 0,
            vendor_id: String::new(),
            model_name: String::new(),
            cpu_family: 0,
            model: 0,
            stepping: 0,
            microcode: String::new(),
            cpu_mhz: 0.0,
            cache_size: 0,
            physical_id: 0,
            siblings: 0,
            core_id: 0,
            cpu_cores: 0,
            apicid: 0,
            initial_apicid: 0,
            fpu: false,
            fpu_exception: false,
            cpuid_level: 0,
            wp: false,
            flags: Vec::<String>::new(),
            bugs: Vec::<String>::new(),
            bogomips: 0.0,
            tlb_size: TlbSize { count: 0, unit: 0 },
            clflush_size: 0,
            cache_alignment: 0,
            address_sizes: AddressSizes { phy: 0, virt: 0 },
            power_management: Vec::<String>::new(),
        }
    }
}

#[derive(Debug)]
pub enum CPUInfo {
    X86_64(Vec<X86_64CpuInfo>),
    Arm64(Vec<Arm64CpuInfo>),
    Unsupported(String),
}
