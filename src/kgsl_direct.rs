use nix::fcntl::open;
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd::close;
use std::fs;

const KGSL_DEVICE: &str = "/dev/kgsl-3d0";
const SYSFS_KGSL: &str = "/sys/class/kgsl/kgsl-3d0";

#[derive(Debug)]
pub struct KgslGpu {
    fd: i32,
}

impl KgslGpu {
    pub fn open() -> Result<Self, String> {
        let path = KGSL_DEVICE;
        if !std::path::Path::new(path).exists() {
            return Err(format!("{} não encontrado", path));
        }

        let fd = open(path, OFlag::O_RDWR, Mode::empty())
            .map_err(|e| format!("Erro ao abrir {}: {}", path, e))?;

        Ok(KgslGpu { fd })
    }

    pub fn get_info(&self) -> Result<KgslInfo, String> {
        let sysfs = std::path::Path::new(SYSFS_KGSL);

        let gpu_model = fs::read_to_string(sysfs.join("gpu_model"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        let clock = fs::read_to_string(sysfs.join("clock_mhz"))
            .and_then(|s| {
                s.trim()
                    .parse::<u32>()
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, ""))
            })
            .unwrap_or(0);

        let max_clock = fs::read_to_string(sysfs.join("max_clock_mhz"))
            .and_then(|s| {
                s.trim()
                    .parse::<u32>()
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, ""))
            })
            .unwrap_or(0);

        let busy = fs::read_to_string(sysfs.join("gpubusy"))
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
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, ""))
            })
            .map(|t| t as f32 / 1000.0)
            .unwrap_or(0.0);

        let freqs = fs::read_to_string(sysfs.join("gpu_available_frequencies"))
            .map(|s| {
                s.split_whitespace()
                    .filter_map(|f| f.parse::<u32>().ok())
                    .map(|f| f / 1_000_000)
                    .collect()
            })
            .unwrap_or_default();

        Ok(KgslInfo {
            model: gpu_model,
            clock,
            max_clock,
            busy_cycles: busy.0,
            total_cycles: busy.1,
            temperature: temp,
            frequencies: freqs,
        })
    }

    pub fn force_clk_on(&self) -> Result<(), String> {
        let path = format!("{}/force_clk_on", SYSFS_KGSL);
        fs::write(&path, "1").map_err(|e| format!("Erro: {}", e))
    }

    pub fn get_power(&self) -> Result<PowerStats, String> {
        let sysfs = std::path::Path::new(SYSFS_KGSL);

        let busy = fs::read_to_string(sysfs.join("gpubusy"))
            .map(|s| {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() >= 2 {
                    parts[0].parse::<u64>().unwrap_or(0)
                } else {
                    0
                }
            })
            .unwrap_or(0);

        let total = fs::read_to_string(sysfs.join("gpubusy"))
            .map(|s| {
                let parts: Vec<&str> = s.split_whitespace().collect();
                if parts.len() >= 2 {
                    parts[1].parse::<u64>().unwrap_or(1)
                } else {
                    1
                }
            })
            .unwrap_or(1);

        let usage_pct = if total > 0 {
            (busy as f64 / total as f64 * 100.0)
        } else {
            0.0
        };

        Ok(PowerStats {
            busy_cycles: busy,
            total_cycles: total,
            usage_percent: usage_pct,
        })
    }
}

impl Drop for KgslGpu {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

#[derive(Debug, Clone)]
pub struct KgslInfo {
    pub model: String,
    pub clock: u32,
    pub max_clock: u32,
    pub busy_cycles: u64,
    pub total_cycles: u64,
    pub temperature: f32,
    pub frequencies: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct PowerStats {
    pub busy_cycles: u64,
    pub total_cycles: u64,
    pub usage_percent: f64,
}

pub fn quick_status() -> Result<String, String> {
    let gpu = KgslGpu::open()?;
    let info = gpu.get_info()?;
    let power = gpu.get_power()?;

    Ok(format!(
        "GPU: {} | Clock: {}MHz/{}MHz | Usage: {:.1}% | Temp: {:.1}°C",
        info.model, info.clock, info.max_clock, power.usage_percent, info.temperature
    ))
}
