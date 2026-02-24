use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionMode {
    KGSL,
    DRM,
    DSP,
    Vulkan,
    OpenCL,
}

impl fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionMode::KGSL => write!(f, "KGSL (Direct GPU)"),
            PermissionMode::DRM => write!(f, "DRM (Direct Rendering)"),
            PermissionMode::DSP => write!(f, "DSP (Digital Signal Processor)"),
            PermissionMode::Vulkan => write!(f, "Vulkan (Compute Shaders)"),
            PermissionMode::OpenCL => write!(f, "OpenCL (Parallel Computing)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    Locked,
    Basic,
    Standard,
    Advanced,
    GodMode,
}

impl fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionLevel::Locked => write!(f, "🔒 LOCKED - Sem acesso"),
            PermissionLevel::Basic => write!(f, "🔓 BASIC - 1 modo disponível"),
            PermissionLevel::Standard => write!(f, "📊 STANDARD - 2 modos disponíveis"),
            PermissionLevel::Advanced => write!(f, "⚡ ADVANCED - 3-4 modos disponíveis"),
            PermissionLevel::GodMode => write!(f, "🌟 GOD MODE - Todos os 5 modos disponíveis"),
        }
    }
}

impl PermissionLevel {
    pub fn rank(&self) -> u8 {
        match self {
            PermissionLevel::Locked => 0,
            PermissionLevel::Basic => 1,
            PermissionLevel::Standard => 2,
            PermissionLevel::Advanced => 3,
            PermissionLevel::GodMode => 4,
        }
    }

    pub fn can_access(&self, mode: &PermissionMode) -> bool {
        let rank = self.rank();
        match mode {
            PermissionMode::KGSL => rank >= 1,
            PermissionMode::DRM => rank >= 2,
            PermissionMode::DSP => rank >= 2,
            PermissionMode::Vulkan => rank >= 3,
            PermissionMode::OpenCL => rank >= 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    None,
    ReadOnly,
    Monitor,
    Compute,
    Full,
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessLevel::None => write!(f, "Nenhum acesso"),
            AccessLevel::ReadOnly => write!(f, "Somente leitura"),
            AccessLevel::Monitor => write!(f, "Monitoramento"),
            AccessLevel::Compute => write!(f, "Compute (execução)"),
            AccessLevel::Full => write!(f, "Acesso total"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModeInfo {
    pub mode: PermissionMode,
    pub available: bool,
    pub permissions: Vec<String>,
    pub capabilities: Vec<String>,
}

impl ModeInfo {
    pub fn new(mode: PermissionMode) -> Self {
        let (available, permissions, capabilities) = match mode {
            PermissionMode::KGSL => (
                std::path::Path::new("/dev/kgsl-3d0").exists(),
                vec!["read".to_string(), "clock_control".to_string()],
                vec![
                    "gpu_info".to_string(),
                    "memory_read".to_string(),
                    "clock_set".to_string(),
                ],
            ),
            PermissionMode::DRM => (
                std::path::Path::new("/dev/dri/card0").exists(),
                vec!["read".to_string(), "write".to_string()],
                vec!["device_info".to_string(), "properties".to_string()],
            ),
            PermissionMode::DSP => (
                std::path::Path::new("/dev/adsprpc-smd").exists(),
                vec!["read".to_string()],
                vec!["dsp_info".to_string()],
            ),
            PermissionMode::Vulkan => (
                std::path::Path::new("/usr/lib/libvulkan_freedreno.so").exists(),
                vec!["init".to_string(), "compute".to_string()],
                vec!["physical_devices".to_string(), "compute_queues".to_string()],
            ),
            PermissionMode::OpenCL => (
                std::path::Path::new("/usr/lib/libOpenCL.so").exists(),
                vec!["init".to_string(), "compute".to_string()],
                vec![
                    "platforms".to_string(),
                    "devices".to_string(),
                    "contexts".to_string(),
                ],
            ),
        };

        ModeInfo {
            mode,
            available,
            permissions,
            capabilities,
        }
    }

    pub fn is_functional(&self) -> bool {
        self.available && !self.capabilities.is_empty()
    }
}

pub fn get_all_modes_info() -> Vec<ModeInfo> {
    vec![
        ModeInfo::new(PermissionMode::KGSL),
        ModeInfo::new(PermissionMode::DRM),
        ModeInfo::new(PermissionMode::DSP),
        ModeInfo::new(PermissionMode::Vulkan),
        ModeInfo::new(PermissionMode::OpenCL),
    ]
}
