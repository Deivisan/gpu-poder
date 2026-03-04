//! GPU PODER - Phase 5: Compute Execution
//! Matrix multiply benchmark: CPU vs GPU

use gpu_poder::{KgslDevice, CommandBuffer, ComputeShader};
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    println!("🦞🔥 GPU PODER - Phase 5: Compute Execution");
    println!("📱 Device: Poco X5 5G (Snapdragon 695)\n");
    
    // 1. Open KGSL device
    let _device = KgslDevice::open()?;
    println!("✅ KGSL device opened");
    
    // 2. Create command buffer
    let cmd_buf = CommandBuffer::new(4096);
    println!("✅ Command buffer created (4KB)");
    
    // 3. Create compute shader
    let mut shader = ComputeShader::new(0);
    shader.set_workgroup_size(256, 1, 1);
    println!("✅ Compute shader created");
    
    // 4. CPU Benchmark (baseline)
    println!("\n=== CPU Benchmark ===");
    let size = 512;
    let start = Instant::now();
    let _result = cpu_matrix_multiply(size);
    let cpu_time = start.elapsed();
    println!("⏱️ CPU time: {:.3}ms", cpu_time.as_secs_f64() * 1000.0);
    
    // 5. GPU Benchmark (placeholder)
    println!("\n=== GPU Benchmark ===");
    let start = Instant::now();
    let _ = shader.matrix_multiply(&cmd_buf, size);
    let gpu_time = start.elapsed();
    println!("⏱️ GPU time: {:.3}ms (estimated)", gpu_time.as_secs_f64() * 1000.0);
    
    // 6. GPU Status
    println!("\n=== GPU Status ===");
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
    
    println!("\n✅ Phase 5 infrastructure ready");
    println!("⏳ Next: Actual GPU command submission");
    
    Ok(())
}

fn cpu_matrix_multiply(size: usize) -> f32 {
    let a = vec![1.0f32; size * size];
    let b = vec![1.0f32; size * size];
    let mut c = vec![0.0f32; size * size];
    
    for i in 0..size {
        for j in 0..size {
            let mut sum = 0.0f32;
            for k in 0..size {
                sum += a[i * size + k] * b[k * size + j];
            }
            c[i * size + j] = sum;
        }
    }
    
    println!("🧮 CPU matrix multiply: {}x{}", size, size);
    c[0]
}
