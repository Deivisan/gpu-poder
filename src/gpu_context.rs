//! GPU Context Management via KGSL
//! Context creation, destruction, and state management

use nix::ioctl_readwrite;
use libc::{c_uint};

// KGSL Context constants
const KGSL_CONTEXT_SAVE_GMEM: c_uint = 0x00000001;
const KGSL_CONTEXT_NO_GMEM_ALLOC: c_uint = 0x00000002;

// Corrected ioctl command numbers from Linux kernel
// KGSL_IOC_DRAWCTXT_CREATE = _IOWR('K', 9, struct kgsl_drawctxt_create)
ioctl_readwrite!(kgsl_drawctxt_create, b'K', 9, KgslDrawctxtCreate);

#[repr(C)]
pub struct KgslDrawctxtCreate {
    pub flags: c_uint,
    pub context_id: c_uint,
}

pub struct GpuContext {
    pub context_id: u32,
    device_fd: i32,
}

impl GpuContext {
    /// Create GPU context
    pub fn create(device_fd: i32, flags: u32) -> std::io::Result<Self> {
        let mut ctx = KgslDrawctxtCreate {
            flags,
            context_id: 0,
        };

        unsafe {
            kgsl_drawctxt_create(device_fd, &mut ctx)
                .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
        }

        Ok(GpuContext {
            context_id: ctx.context_id,
            device_fd,
        })
    }
}

impl Drop for GpuContext {
    fn drop(&mut self) {
        // Context cleanup would go here
    }
}
