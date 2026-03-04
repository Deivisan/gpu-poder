//! GPU Command Buffer Management
//! Command submission and execution

use libc::{c_uint, c_ulong};

// Command buffer constants
const KGSL_CMD_FLAGS_PMODE: c_uint = 0x00000001;
const KGSL_CMD_FLAGS_INTERNAL_ISSUE: c_uint = 0x00000002;

#[repr(C)]
pub struct KgslCommandObject {
    pub offset: c_ulong,
    pub size: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
pub struct KgslCommandBuffer {
    pub context_id: c_uint,
    pub flags: c_uint,
    pub cmds: *mut KgslCommandObject,
    pub num_cmds: c_uint,
    pub timestamp: c_uint,
}

pub struct CommandBuffer {
    pub buffer: Vec<u8>,
    pub size: usize,
}

impl CommandBuffer {
    /// Create new command buffer
    pub fn new(size: usize) -> Self {
        CommandBuffer {
            buffer: vec![0u8; size],
            size,
        }
    }

    /// Add command to buffer
    pub fn add_command(&mut self, cmd: &[u8]) -> std::io::Result<()> {
        if self.buffer.len() + cmd.len() > self.size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Command buffer overflow",
            ));
        }
        self.buffer.extend_from_slice(cmd);
        Ok(())
    }

    /// Get buffer pointer
    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    /// Get buffer size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
