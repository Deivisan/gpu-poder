//! GPU Monitor - Real-time status
use std::fs;

fn main() {
    println!("🦞🔥 GPU Monitor - Adreno 619");
    
    // KGSL status
    if let Ok(busy) = fs::read_to_string("/sys/class/kgsl/kgsl-3d0/gpubusy") {
        let parts: Vec<&str> = busy.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            let busy_val: u64 = parts[0].parse().unwrap_or(0);
            let total_val: u64 = parts[1].parse().unwrap_or(1);
            let util = (busy_val as f64 / total_val as f64 * 100.0) as u32;
            println!("📊 GPU Utilization: {}%", util);
        }
    }
    
    // Clock frequency
    if let Ok(clock) = fs::read_to_string("/sys/class/kgsl/kgsl-3d0/clock_mhz") {
        println!("⚡ Clock: {} MHz", clock.trim());
    }
    
    // Thermal
    if let Ok(temp) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(t) = temp.trim().parse::<u32>() {
            println!("🌡️ Thermal: {}°C", t / 1000);
        }
    }
}
