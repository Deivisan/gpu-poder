use crate::modes::PermissionMode;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ComputeAccess {
    mode: PermissionMode,
    is_available: bool,
    library_path: String,
}

impl ComputeAccess {
    pub fn new(mode: PermissionMode) -> Result<Self, String> {
        let (library_path, is_available) = match mode {
            PermissionMode::Vulkan => {
                let path = "/usr/lib/libvulkan_freedreno.so";
                (path.to_string(), PathBuf::from(path).exists())
            }
            PermissionMode::OpenCL => {
                let path = "/usr/lib/libOpenCL.so";
                (path.to_string(), PathBuf::from(path).exists())
            }
            _ => return Err("Modo inválido para ComputeAccess".to_string()),
        };

        Ok(ComputeAccess {
            mode,
            is_available,
            library_path,
        })
    }

    pub fn is_functional(&self) -> bool {
        self.is_available
    }

    pub fn get_info(&self) -> serde_json::Value {
        serde_json::json!({
            "mode": format!("{:?}", self.mode),
            "library_path": self.library_path,
            "is_available": self.is_available,
            "can_compute": self.is_available,
        })
    }

    pub fn check_vulkan_devices(&self) -> Result<Vec<String>, String> {
        if self.mode != PermissionMode::Vulkan {
            return Err("Não é modo Vulkan".to_string());
        }

        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.aarch64.json vulkaninfo 2>/dev/null | grep -i 'GPU id' | head -5")
            .output()
            .map_err(|e| format!("Falha ao executar vulkaninfo: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let devices: Vec<String> = output_str
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.trim().to_string())
            .collect();

        Ok(devices)
    }

    pub fn check_opencl_platforms(&self) -> Result<Vec<String>, String> {
        if self.mode != PermissionMode::OpenCL {
            return Err("Não é modo OpenCL".to_string());
        }

        let output = std::process::Command::new("clinfo")
            .output()
            .map_err(|e| format!("Falha ao executar clinfo: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let platforms: Vec<String> = output_str
            .lines()
            .filter(|l| l.contains("Platform Name"))
            .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
            .collect();

        if platforms.is_empty() {
            Ok(vec!["Nenhuma plataforma OpenCL disponível".to_string()])
        } else {
            Ok(platforms)
        }
    }
}

pub fn try_vulkan_init() -> Result<bool, String> {
    let path = PathBuf::from("/usr/lib/libvulkan_freedreno.so");
    if path.exists() {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.aarch64.json vulkaninfo 2>&1 | head -20")
            .output()
            .map_err(|e| format!("Erro: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(output_str.contains("Vulkan Instance"))
    } else {
        Ok(false)
    }
}

pub fn try_opencl_init() -> Result<bool, String> {
    let path = PathBuf::from("/usr/lib/libOpenCL.so");
    if path.exists() {
        Ok(true)
    } else {
        Ok(false)
    }
}
