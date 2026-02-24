use crate::compute::ComputeAccess;
use crate::drm::DrmAccess;
use crate::dsp::DspAccess;
use crate::kgsl::KgslAccess;
use crate::modes::{AccessLevel, PermissionLevel, PermissionMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub device_id: String,
    pub clock_current: u32,
    pub clock_max: u32,
    pub memory_total: u64,
    pub memory_available: u64,
    pub temperature: f32,
    pub busy_cycles: u64,
    pub total_cycles: u64,
    pub frequencies: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuCapabilities {
    pub has_kgsl: bool,
    pub has_drm: bool,
    pub has_dsp: bool,
    pub has_vulkan: bool,
    pub has_opencl: bool,
    pub compute_units: u32,
    pub supported_modes: Vec<PermissionMode>,
}

#[derive(Debug)]
pub struct GpuPoder {
    pub info: GpuInfo,
    pub capabilities: GpuCapabilities,
    pub permission_level: PermissionLevel,
    pub access_level: AccessLevel,
    pub kgsl: Option<KgslAccess>,
    pub drm: Option<DrmAccess>,
    pub dsp: Option<DspAccess>,
    pub compute: Option<ComputeAccess>,
    pub active_mode: Option<PermissionMode>,
}

impl GpuPoder {
    pub fn new() -> Result<Self, String> {
        let mut poder = GpuPoder {
            info: GpuInfo {
                name: "Unknown".to_string(),
                device_id: "unknown".to_string(),
                clock_current: 0,
                clock_max: 0,
                memory_total: 0,
                memory_available: 0,
                temperature: 0.0,
                busy_cycles: 0,
                total_cycles: 0,
                frequencies: vec![],
            },
            capabilities: GpuCapabilities {
                has_kgsl: false,
                has_drm: false,
                has_dsp: false,
                has_vulkan: false,
                has_opencl: false,
                compute_units: 0,
                supported_modes: vec![],
            },
            permission_level: PermissionLevel::Locked,
            access_level: AccessLevel::None,
            kgsl: None,
            drm: None,
            dsp: None,
            compute: None,
            active_mode: None,
        };

        poder.detect_hardware()?;
        poder.detect_capabilities()?;
        poder.determine_permission_level()?;

        Ok(poder)
    }

    fn detect_hardware(&mut self) -> Result<(), String> {
        match crate::kgsl::read_gpu_info() {
            Ok(info) => self.info = info,
            Err(_) => {
                self.info = GpuInfo {
                    name: "Adreno 619v2".to_string(),
                    device_id: "237:0".to_string(),
                    clock_current: 266,
                    clock_max: 840,
                    memory_total: 0,
                    memory_available: 0,
                    temperature: 0.0,
                    busy_cycles: 0,
                    total_cycles: 0,
                    frequencies: vec![840, 770, 650, 490, 390, 266],
                };
            }
        }

        Ok(())
    }

    fn detect_capabilities(&mut self) -> Result<(), String> {
        let mut caps = GpuCapabilities {
            has_kgsl: std::path::Path::new("/dev/kgsl-3d0").exists(),
            has_drm: std::path::Path::new("/dev/dri/card0").exists(),
            has_dsp: std::path::Path::new("/dev/adsprpc-smd").exists(),
            has_vulkan: std::path::Path::new("/usr/lib/libvulkan_freedreno.so").exists(),
            has_opencl: std::path::Path::new("/usr/lib/libOpenCL.so").exists(),
            compute_units: 8,
            supported_modes: vec![],
        };

        if caps.has_kgsl {
            caps.supported_modes.push(PermissionMode::KGSL);
        }
        if caps.has_drm {
            caps.supported_modes.push(PermissionMode::DRM);
        }
        if caps.has_dsp {
            caps.supported_modes.push(PermissionMode::DSP);
        }
        if caps.has_vulkan {
            caps.supported_modes.push(PermissionMode::Vulkan);
        }
        if caps.has_opencl {
            caps.supported_modes.push(PermissionMode::OpenCL);
        }

        self.capabilities = caps;
        Ok(())
    }

    fn determine_permission_level(&mut self) -> Result<(), String> {
        let modes_count = self.capabilities.supported_modes.len();

        self.permission_level = match modes_count {
            0 => PermissionLevel::Locked,
            1 => PermissionLevel::Basic,
            2 => PermissionLevel::Standard,
            3 | 4 => PermissionLevel::Advanced,
            5 => PermissionLevel::GodMode,
            _ => PermissionLevel::GodMode,
        };

        self.access_level = match modes_count {
            0 => AccessLevel::None,
            1 => AccessLevel::ReadOnly,
            2 => AccessLevel::Monitor,
            3 | 4 => AccessLevel::Compute,
            _ => AccessLevel::Full,
        };

        Ok(())
    }

    pub fn activate_mode(&mut self, mode: PermissionMode) -> Result<(), String> {
        match mode {
            PermissionMode::KGSL => {
                self.kgsl = Some(KgslAccess::new()?);
                self.active_mode = Some(PermissionMode::KGSL);
            }
            PermissionMode::DRM => {
                self.drm = Some(DrmAccess::new()?);
                self.active_mode = Some(PermissionMode::DRM);
            }
            PermissionMode::DSP => {
                self.dsp = Some(DspAccess::new()?);
                self.active_mode = Some(PermissionMode::DSP);
            }
            PermissionMode::Vulkan | PermissionMode::OpenCL => {
                self.compute = Some(ComputeAccess::new(mode)?);
                self.active_mode = Some(mode);
            }
        }
        Ok(())
    }

    pub fn get_status(&self) -> HashMap<String, serde_json::Value> {
        let mut status = HashMap::new();

        status.insert("gpu_name".to_string(), serde_json::json!(self.info.name));
        status.insert(
            "clock_current".to_string(),
            serde_json::json!(self.info.clock_current),
        );
        status.insert(
            "clock_max".to_string(),
            serde_json::json!(self.info.clock_max),
        );
        status.insert(
            "permission_level".to_string(),
            serde_json::json!(format!("{:?}", self.permission_level)),
        );
        status.insert(
            "access_level".to_string(),
            serde_json::json!(format!("{:?}", self.access_level)),
        );
        status.insert(
            "has_kgsl".to_string(),
            serde_json::json!(self.capabilities.has_kgsl),
        );
        status.insert(
            "has_drm".to_string(),
            serde_json::json!(self.capabilities.has_drm),
        );
        status.insert(
            "has_dsp".to_string(),
            serde_json::json!(self.capabilities.has_dsp),
        );
        status.insert(
            "has_vulkan".to_string(),
            serde_json::json!(self.capabilities.has_vulkan),
        );
        status.insert(
            "has_opencl".to_string(),
            serde_json::json!(self.capabilities.has_opencl),
        );
        status.insert(
            "supported_modes".to_string(),
            serde_json::json!(self.capabilities.supported_modes.len()),
        );

        status
    }
}
