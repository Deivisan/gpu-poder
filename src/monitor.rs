use gpu_poder::init;
use gpu_poder::kgsl_direct::KgslGpu;
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║              GPU PODER - MONITORAMENTO EM TEMPO REAL              ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    let poder = init().expect("Falha ao inicializar GPU PODER");

    println!("📊 INFORMAÇÕES DA GPU");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Nome:         {}", poder.info.name);
    println!("  Device ID:    {}", poder.info.device_id);
    println!("  Clock:        {} MHz", poder.info.clock_current);
    println!("  Clock Máx:    {} MHz", poder.info.clock_max);
    println!("  Temperatura:   {:.1}°C", poder.info.temperature);
    println!("  Frequências:  {:?}", poder.info.frequencies);
    println!();

    println!("📈 USO DA GPU (tem que monitorar em tempo real)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  Pressione Ctrl+C para sair\n");

    println!("┌─────────────┬────────────┬────────────┬────────────┐");
    println!("│   Clock     │   Usage    │  Busy/Tot  │    Temp    │");
    println!("├─────────────┼────────────┼────────────┼────────────┤");

    loop {
        match KgslGpu::open() {
            Ok(gpu) => {
                if let Ok(info) = gpu.get_info() {
                    if let Ok(power) = gpu.get_power() {
                        println!(
                            "│ {:>9} │ {:>9.1}% │ {:>9} │ {:>9.1}°C │",
                            format!("{}MHz", info.clock),
                            power.usage_percent,
                            format!("{}/{}", power.busy_cycles, power.total_cycles),
                            info.temperature
                        );
                    }
                }
            }
            Err(e) => {
                println!("│ Erro: {}                                           │", e);
            }
        }

        println!("└─────────────┴────────────┴────────────┴────────────┘");
        thread::sleep(Duration::from_secs(2));
        print!("\x1b[5A");
    }
}
