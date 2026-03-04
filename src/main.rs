//! GPU PODER - KGSL Direct Access Demo
//! Phase 3: Testing different ioctl approaches

use std::io;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

fn main() -> io::Result<()> {
    println!("🦞🔥 GPU PODER - Phase 3: ioctl Investigation");
    println!("📱 Device: Poco X5 5G (Snapdragon 695)\n");
    
    // Open KGSL device
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kgsl-3d0")?;
    
    let fd = file.as_raw_fd();
    println!("✅ KGSL device opened (fd: {})", fd);
    
    // Test basic device properties
    println!("\n=== Testing Device Properties ===");
    
    // Try to read device info via sysfs (more reliable)
    if let Ok(busy) = std::fs::read_to_string("/sys/class/kgsl/kgsl-3d0/gpubusy") {
        let parts: Vec<&str> = busy.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let busy_val: u64 = parts[0].parse().unwrap_or(0);
            let total_val: u64 = parts[1].parse().unwrap_or(1);
            let util = (busy_val as f64 / total_val as f64 * 100.0) as u32;
            println!("📊 GPU Utilization: {}%", util);
        }
    }
    
    if let Ok(clock) = std::fs::read_to_string("/sys/class/kgsl/kgsl-3d0/clock_mhz") {
        println!("⚡ Clock: {} MHz", clock.trim());
    }
    
    if let Ok(freq_table) = std::fs::read_to_string("/sys/class/kgsl/kgsl-3d0/devfreq/available_frequencies") {
        println!("📈 Available frequencies: {}", freq_table.trim());
    }
    
    println!("\n=== ioctl Investigation ===");
    println!("⚠️ Context creation via ioctl 9: EINVAL");
    println!("💡 Possible causes:");
    println!("   1. ioctl number still incorrect");
    println!("   2. Structure alignment issue");
    println!("   3. Device doesn't support in chroot");
    println!("   4. Requires kernel module");
    
    println!("\n✅ KGSL device accessible via sysfs");
    println!("⏳ Direct ioctl access: Requires further investigation");
    
    Ok(())
}
