# Phase 6: GPU Command Submission - Plan

## Problem: ioctl EINVAL on context creation

### Root Cause Analysis
1. ioctl number might still be wrong
2. Structure alignment issue
3. Device doesn't support in chroot
4. Requires kernel module

### Solution: Use libdrm instead of raw ioctl

libdrm provides:
- Pre-built KGSL bindings
- Proper structure alignment
- Error handling
- Tested on Adreno devices

### Implementation Plan
1. Install libdrm-dev
2. Create Rust bindings for libdrm
3. Use libdrm for GPU context creation
4. Test GPU memory allocation
5. Submit compute commands

### Alternative: Use sysfs + direct GPU access
- Monitor GPU via sysfs (already working)
- Use GPU compute via alternative method
- Document limitations

---
**Status:** Ready to implement Phase 6
**Approach:** Try libdrm first, fallback to sysfs monitoring
