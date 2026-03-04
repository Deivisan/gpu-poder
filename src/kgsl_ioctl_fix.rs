//! KGSL ioctl command numbers - Corrected
//! Based on Linux kernel drivers/gpu/drm/msm/kgsl/

use nix::ioctl_readwrite;
use libc::{c_uint, c_ulong};

// Correct KGSL ioctl command numbers
// Format: _IOC(dir, type, nr, size)
// dir: 0=none, 1=write, 2=read, 3=read/write

// KGSL_IOC_DEVICE_GETPROPERTY = _IOWR('K', 2, struct kgsl_device_getproperty)
ioctl_readwrite!(kgsl_device_getproperty, b'K', 2, KgslDeviceGetproperty);

// KGSL_IOC_DEVICE_SETPROPERTY = _IOW('K', 3, struct kgsl_device_setproperty)
// ioctl_write!(kgsl_device_setproperty, b'K', 3, KgslDeviceSetproperty);

// KGSL_IOC_DEVICE_WAITTIMESTAMP = _IOW('K', 4, struct kgsl_device_waittimestamp)
// ioctl_write!(kgsl_device_waittimestamp, b'K', 4, KgslDeviceWaittimestamp);

// KGSL_IOC_RINGBUFFER_ISSUEIBCMDS = _IOWR('K', 5, struct kgsl_ringbuffer_issueibcmds)
ioctl_readwrite!(kgsl_ringbuffer_issueibcmds, b'K', 5, KgslRingbufferIssueibcmds);

// KGSL_IOC_SUBMIT_CMDS = _IOWR('K', 6, struct kgsl_submit_cmds)
ioctl_readwrite!(kgsl_submit_cmds, b'K', 6, KgslSubmitCmds);

// KGSL_IOC_CMDSTREAM_READTIMESTAMP = _IOWR('K', 7, struct kgsl_cmdstream_readtimestamp)
ioctl_readwrite!(kgsl_cmdstream_readtimestamp, b'K', 7, KgslCmdstreamReadtimestamp);

// KGSL_IOC_CMDSTREAM_FREEMEMONTIMESTAMP = _IOW('K', 8, struct kgsl_cmdstream_freememontimestamp)
// ioctl_write!(kgsl_cmdstream_freememontimestamp, b'K', 8, KgslCmdstreamFreememontimestamp);

// KGSL_IOC_DRAWCTXT_CREATE = _IOWR('K', 9, struct kgsl_drawctxt_create)
ioctl_readwrite!(kgsl_drawctxt_create, b'K', 9, KgslDrawctxtCreate);

// KGSL_IOC_DRAWCTXT_DESTROY = _IOW('K', 10, struct kgsl_drawctxt_destroy)
// ioctl_write!(kgsl_drawctxt_destroy, b'K', 10, KgslDrawctxtDestroy);

#[repr(C)]
pub struct KgslDeviceGetproperty {
    pub type_: c_uint,
    pub value: c_ulong,
    pub sizebytes: c_uint,
}

#[repr(C)]
pub struct KgslRingbufferIssueibcmds {
    pub drawctxt_id: c_uint,
    pub ibdesc_addr: c_ulong,
    pub numibs: c_uint,
    pub timestamp: c_uint,
}

#[repr(C)]
pub struct KgslSubmitCmds {
    pub context_id: c_uint,
    pub flags: c_uint,
    pub numcmds: c_uint,
    pub numobjs: c_uint,
    pub cmds: c_ulong,
    pub objs: c_ulong,
    pub syncobj: c_ulong,
}

#[repr(C)]
pub struct KgslCmdstreamReadtimestamp {
    pub type_: c_uint,
    pub timestamp: c_uint,
}

#[repr(C)]
pub struct KgslDrawctxtCreate {
    pub flags: c_uint,
    pub context_id: c_uint,
}
