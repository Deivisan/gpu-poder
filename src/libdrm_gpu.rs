//! libdrm GPU Access - Alternative to raw ioctl
//! Uses libdrm for proper KGSL device management

use libc::{c_int, c_uint, c_void};

// libdrm function declarations
extern "C" {
    pub fn drmOpen(name: *const u8, busid: *const u8) -> c_int;
    pub fn drmClose(fd: c_int) -> c_int;
    pub fn drmGetVersion(fd: c_int) -> *mut DrmVersion;
    pub fn drmFreeVersion(version: *mut DrmVersion);
}

#[repr(C)]
pub struct DrmVersion {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub version_patchlevel: c_int,
    pub name_len: c_uint,
    pub name: *mut u8,
    pub date_len: c_uint,
    pub date: *mut u8,
    pub desc_len: c_uint,
    pub desc: *mut u8,
}

pub struct LibdrmDevice {
    fd: c_int,
}

impl LibdrmDevice {
    /// Open DRM device via libdrm
    pub fn open() -> std::io::Result<Self> {
        unsafe {
            let fd = drmOpen(b"msm\0".as_ptr(), std::ptr::null());
            if fd < 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Failed to open DRM device",
                ));
            }
            Ok(LibdrmDevice { fd })
        }
    }

    /// Get DRM version
    pub fn get_version(&self) -> std::io::Result<String> {
        unsafe {
            let version = drmGetVersion(self.fd);
            if version.is_null() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to get DRM version",
                ));
            }

            let name_slice = std::slice::from_raw_parts(
                (*version).name as *const u8,
                (*version).name_len as usize,
            );
            let name = String::from_utf8_lossy(name_slice).to_string();

            drmFreeVersion(version);
            Ok(name)
        }
    }
}

impl Drop for LibdrmDevice {
    fn drop(&mut self) {
        unsafe {
            drmClose(self.fd);
        }
    }
}
