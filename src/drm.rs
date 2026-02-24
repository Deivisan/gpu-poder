use std::fs;
use std::path::PathBuf;

const DRM_DEVICE: &str = "/dev/dri/card0";
const SYSFS_DRM: &str = "/sys/class/drm/card0";

#[derive(Debug)]
pub struct DrmAccess {
    device_fd: Option<i32>,
    device_path: PathBuf,
}

impl DrmAccess {
    pub fn new() -> Result<Self, String> {
        let path = PathBuf::from(DRM_DEVICE);
        if !path.exists() {
            return Err(format!("Device DRM não encontrado: {}", DRM_DEVICE));
        }

        Ok(DrmAccess {
            device_fd: None,
            device_path: path,
        })
    }

    pub fn read_driver(&self) -> Result<String, String> {
        let path = PathBuf::from(SYSFS_DRM).join("device/driver");
        if path.exists() {
            fs::read_to_string(path.join("name"))
                .map(|s| s.trim().to_string())
                .map_err(|e| format!("Falha ao ler driver: {}", e))
        } else {
            Ok("msm_drm".to_string())
        }
    }

    pub fn read_device_name(&self) -> Result<String, String> {
        let path = PathBuf::from(SYSFS_DRM).join("device/name");
        fs::read_to_string(&path)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Falha ao ler nome do dispositivo: {}", e))
    }

    pub fn list_available_modes(&self) -> Result<Vec<String>, String> {
        let path = PathBuf::from(SYSFS_DRM);
        let mut modes = Vec::new();

        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("card0-") {
                    modes.push(name);
                }
            }
        }

        Ok(modes)
    }

    pub fn get_device_info(&self) -> Result<serde_json::Value, String> {
        let base = PathBuf::from(SYSFS_DRM).join("device");

        let driver = self.read_driver().unwrap_or_else(|_| "unknown".to_string());
        let name = self
            .read_device_name()
            .unwrap_or_else(|_| "unknown".to_string());
        let modes = self.list_available_modes().unwrap_or_default();

        Ok(serde_json::json!({
            "driver": driver,
            "device_name": name,
            "available_modes": modes,
            "device_path": DRM_DEVICE,
        }))
    }
}

pub fn check_drm_available() -> bool {
    PathBuf::from(DRM_DEVICE).exists()
}
