use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd::read;
use std::fs;
use std::path::PathBuf;

const KGSL_DEVICE: &str = "/dev/kgsl-3d0";
const SYSFS_KGSL: &str = "/sys/class/kgsl/kgsl-3d0";

#[derive(Debug)]
pub struct KgslAccess {
    device_fd: Option<i32>,
    device_path: PathBuf,
}

impl KgslAccess {
    pub fn new() -> Result<Self, String> {
        let path = PathBuf::from(KGSL_DEVICE);
        if !path.exists() {
            return Err(format!("Device KGSL não encontrado: {}", KGSL_DEVICE));
        }

        let fd = open(&path, OFlag::O_RDWR, Mode::empty())
            .map_err(|e| format!("Falha ao abrir KGSL: {}", e))?;

        Ok(KgslAccess {
            device_fd: Some(fd),
            device_path: path,
        })
    }

    pub fn read_clock(&self) -> Result<u32, String> {
        let path = SYSFS_KGSL.to_string() + "/clock_mhz";
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Falha ao ler clock: {}", e))?;
        content
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Falha ao converter clock: {}", e))
    }

    pub fn read_max_clock(&self) -> Result<u32, String> {
        let path = SYSFS_KGSL.to_string() + "/max_clock_mhz";
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Falha ao ler max_clock: {}", e))?;
        content
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Falha ao converter max_clock: {}", e))
    }

    pub fn read_busy(&self) -> Result<(u64, u64), String> {
        let path = SYSFS_KGSL.to_string() + "/gpubusy";
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Falha ao ler gpubusy: {}", e))?;
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 2 {
            let busy: u64 = parts[0].parse().unwrap_or(0);
            let total: u64 = parts[1].parse().unwrap_or(1);
            Ok((busy, total))
        } else {
            Err("Formato inválido de gpubusy".to_string())
        }
    }

    pub fn read_frequencies(&self) -> Result<Vec<u32>, String> {
        let path = SYSFS_KGSL.to_string() + "/gpu_available_frequencies";
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Falha ao ler frequências: {}", e))?;
        let mut freqs = Vec::new();
        for freq_str in content.split_whitespace() {
            if let Ok(freq) = freq_str.parse::<u32>() {
                freqs.push(freq / 1_000_000);
            }
        }
        Ok(freqs)
    }

    pub fn read_temperature(&self) -> Result<f32, String> {
        let path = SYSFS_KGSL.to_string() + "/temp";
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Falha ao ler temperatura: {}", e))?;
        let temp: i32 = content
            .trim()
            .parse()
            .map_err(|e| format!("Falha ao converter temperatura: {}", e))?;
        Ok(temp as f32 / 1000.0)
    }

    pub fn read_gpu_model(&self) -> Result<String, String> {
        let path = SYSFS_KGSL.to_string() + "/gpu_model";
        fs::read_to_string(&path)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Falha ao ler modelo GPU: {}", e))
    }

    pub fn set_clock(&self, _mhz: u32) -> Result<(), String> {
        let path = SYSFS_KGSL.to_string() + "/force_clk_on";
        fs::write(&path, "1\n").map_err(|e| format!("Falha ao forçar clock: {}", e))?;
        Ok(())
    }
}

pub fn read_gpu_info() -> Result<super::core::GpuInfo, String> {
    let sysfs = PathBuf::from(SYSFS_KGSL);
    if !sysfs.exists() {
        return Err("Sysfs KGSL não encontrado".to_string());
    }

    let name = fs::read_to_string(sysfs.join("gpu_model"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Unknown GPU".to_string());

    let clock_current = fs::read_to_string(sysfs.join("clock_mhz"))
        .and_then(|s| {
            s.trim()
                .parse::<u32>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "parse error"))
        })
        .unwrap_or(0);

    let clock_max = fs::read_to_string(sysfs.join("max_clock_mhz"))
        .and_then(|s| {
            s.trim()
                .parse::<u32>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "parse error"))
        })
        .unwrap_or(0);

    let frequencies = fs::read_to_string(sysfs.join("gpu_available_frequencies"))
        .map(|s| {
            s.split_whitespace()
                .filter_map(|f| f.parse::<u32>().ok())
                .map(|f| f / 1_000_000)
                .collect()
        })
        .unwrap_or_else(|_| vec![]);

    let gpubusy = fs::read_to_string(sysfs.join("gpubusy"))
        .map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                (
                    parts[0].parse::<u64>().unwrap_or(0),
                    parts[1].parse::<u64>().unwrap_or(1),
                )
            } else {
                (0, 1)
            }
        })
        .unwrap_or((0, 1));

    let temp = fs::read_to_string(sysfs.join("temp"))
        .and_then(|s| {
            s.trim()
                .parse::<i32>()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "parse error"))
        })
        .map(|t| t as f32 / 1000.0)
        .unwrap_or(0.0);

    Ok(super::core::GpuInfo {
        name,
        device_id: "237:0".to_string(),
        clock_current,
        clock_max,
        memory_total: 0,
        memory_available: 0,
        temperature: temp,
        busy_cycles: gpubusy.0,
        total_cycles: gpubusy.1,
        frequencies,
    })
}

impl Drop for KgslAccess {
    fn drop(&mut self) {
        if let Some(fd) = self.device_fd {
            let _ = nix::unistd::close(fd);
        }
    }
}
