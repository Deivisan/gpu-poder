//! KGSL Direct GPU Access via ioctl()
//! Adreno 619 compute shader submission

use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use nix::ioctl_readwrite;
use libc::{c_uint, c_ulong};

// KGSL Device constants
const KGSL_DEVICE_GPU: c_uint = 0;
const KGSL_CONTEXT_SAVE_GMEM: c_uint = 0x00000001;

// KGSL ioctl commands
ioctl_readwrite!(kgsl_device_getproperty, b'K', 2, KgslDeviceGetproperty);
ioctl_readwrite!(kgsl_gpu_command, b'K', 5, KgslGpuCommand);

#[repr(C)]
pub struct KgslDeviceGetproperty {
    pub type_: c_uint,
    pub value: c_ulong,
    pub sizebytes: c_uint,
}

#[repr(C)]
pub struct KgslGpuCommand {
    pub context_id: c_uint,
    pub issueibcmds: c_uint,
    pub numibs: c_uint,
    pub timestamp: c_uint,
}

pub struct KgslDevice {
    fd: i32,
}

impl KgslDevice {
    pub fn open() -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/kgsl-3d0")?;
        
        Ok(KgslDevice {
            fd: file.as_raw_fd(),
        })
    }

    pub fn get_property(&self, prop_type: c_uint) -> std::io::Result<c_ulong> {
        let mut prop = KgslDeviceGetproperty {
            type_: prop_type,
            value: 0,
            sizebytes: 8,
        };

        unsafe {
            kgsl_device_getproperty(self.fd, &mut prop)
                .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
        }

        Ok(prop.value)
    }

    pub fn submit_command(&self, context_id: c_uint, numibs: c_uint) -> std::io::Result<c_uint> {
        let mut cmd = KgslGpuCommand {
            context_id,
            issueibcmds: 0,
            numibs,
            timestamp: 0,
        };

        unsafe {
            kgsl_gpu_command(self.fd, &mut cmd)
                .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
        }

        Ok(cmd.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kgsl_open() {
        match KgslDevice::open() {
            Ok(dev) => println!("✅ KGSL device opened"),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
}
