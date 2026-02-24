// gpu-poder/src/lib.rs
//! # GPU PODER - Arquitetura de Acesso GPU com 5 Modos de Permissão
//! 
//! Este crate fornece uma arquitetura flexível para acessar a GPU de várias formas,
//! com 5 níveis de permissão e múltiplos caminhos de acesso.

pub mod core;
pub mod kgsl;
pub mod kgsl_direct;
pub mod drm;
pub mod dsp;
pub mod compute;
pub mod modes;

pub use core::{GpuPoder, GpuInfo, GpuCapabilities};
pub use modes::{PermissionMode, PermissionLevel, AccessLevel};

use log::info;

pub fn init() -> Result<GpuPoder, String> {
    info!("🚀 Inicializando GPU PODER - Arquitetura de 5 modos");
    GpuPoder::new()
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
