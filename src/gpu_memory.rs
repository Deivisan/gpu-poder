//! GPU Memory Management via KGSL
//! Allocation, mapping, and synchronization

use nix::ioctl_readwrite;
use libc::{c_uint, c_ulong};

// KGSL Memory allocation constants
const KGSL_MEMTYPE_ASHMEM: c_uint = 0x1;
const KGSL_MEMTYPE_KMEM: c_uint = 0x2;
const KGSL_MEMTYPE_PMEM: c_uint = 0x4;

ioctl_readwrite!(kgsl_gpumem_alloc, b'K', 3, KgslGpumemAlloc);
ioctl_readwrite!(kgsl_gpumem_free, b'K', 4, KgslGpumemFree);

#[repr(C)]
pub struct KgslGpumemAlloc {
    pub gpuaddr: c_ulong,
    pub size: c_ulong,
    pub flags: c_uint,
}

#[repr(C)]
pub struct KgslGpumemFree {
    pub gpuaddr: c_ulong,
}

pub struct GpuMemory {
    pub gpu_addr: u64,
    pub size: u64,
    pub cpu_addr: *mut u8,
}

impl GpuMemory {
    /// Allocate GPU memory
    pub fn allocate(device_fd: i32, size: u64) -> std::io::Result<Self> {
        let mut alloc = KgslGpumemAlloc {
            gpuaddr: 0,
            size,
            flags: KGSL_MEMTYPE_KMEM,
        };

        unsafe {
            kgsl_gpumem_alloc(device_fd, &mut alloc)
                .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
        }

        Ok(GpuMemory {
            gpu_addr: alloc.gpuaddr,
            size,
            cpu_addr: std::ptr::null_mut(),
        })
    }

    /// Free GPU memory
    pub fn free(&self, device_fd: i32) -> std::io::Result<()> {
        let free_cmd = KgslGpumemFree {
            gpuaddr: self.gpu_addr,
        };

        unsafe {
            kgsl_gpumem_free(device_fd, &free_cmd as *const _ as *mut _)
                .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
        }

        Ok(())
    }
}
