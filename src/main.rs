use gpu_poder::{init, modes::get_all_modes_info, GpuPoder};
use log::{error, info, LevelFilter};
use std::env;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║           GPU PODER - ARQUITETURA DE ACESSO COMPLETO              ║");
    println!("║              5 Modos de Permissão x Níveis de Uso               ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    match init() {
        Ok(mut poder) => {
            println!("✅ Sistema GPU PODER inicializado com sucesso!\n");

            println!("═══════════════════════════════════════════════════════════════════");
            println!("📊 STATUS DO HARDWARE");
            println!("═══════════════════════════════════════════════════════════════════");
            println!("  GPU:           {}", poder.info.name);
            println!("  Device ID:     {}", poder.info.device_id);
            println!("  Clock Atual:   {} MHz", poder.info.clock_current);
            println!("  Clock Máximo:   {} MHz", poder.info.clock_max);
            println!("  Temperatura:    {:.1}°C", poder.info.temperature);
            println!("  Frequências:   {:?} MHz", poder.info.frequencies);
            println!(
                "  Uso GPU:       {} / {} cycles",
                poder.info.busy_cycles, poder.info.total_cycles
            );
            println!();

            println!("═══════════════════════════════════════════════════════════════════");
            println!("🔐 NÍVEL DE PERMISSÃO");
            println!("═══════════════════════════════════════════════════════════════════");
            println!("  {}", poder.permission_level);
            println!("  Acesso: {}", poder.access_level);
            println!();

            println!("═══════════════════════════════════════════════════════════════════");
            println!("🎯 MODOS DE ACESSO DISPONÍVEIS");
            println!("═══════════════════════════════════════════════════════════════════");

            let modes = get_all_modes_info();
            for mode_info in &modes {
                let status = if mode_info.available { "✅" } else { "❌" };
                println!(
                    "  {} {:?} - {} caps",
                    status,
                    mode_info.mode,
                    mode_info.capabilities.len()
                );
            }
            println!();

            println!("═══════════════════════════════════════════════════════════════════");
            println!("🖥️  DISPONIBILIDADE DE RECURSOS");
            println!("═══════════════════════════════════════════════════════════════════");
            println!(
                "  KGSL:    {}",
                if poder.capabilities.has_kgsl {
                    "✅ Disponível"
                } else {
                    "❌ Indisponível"
                }
            );
            println!(
                "  DRM:     {}",
                if poder.capabilities.has_drm {
                    "✅ Disponível"
                } else {
                    "❌ Indisponível"
                }
            );
            println!(
                "  DSP:     {}",
                if poder.capabilities.has_dsp {
                    "✅ Disponível"
                } else {
                    "❌ Indisponível"
                }
            );
            println!(
                "  Vulkan:  {}",
                if poder.capabilities.has_vulkan {
                    "✅ Instalado"
                } else {
                    "❌ Não instalado"
                }
            );
            println!(
                "  OpenCL:  {}",
                if poder.capabilities.has_opencl {
                    "✅ Instalado"
                } else {
                    "❌ Não instalado"
                }
            );
            println!();

            println!("═══════════════════════════════════════════════════════════════════");
            println!("📈 TABELA DE PERMISSÕES");
            println!("═══════════════════════════════════════════════════════════════════");
            println!("  Nível        | KGSL | DRM | DSP | Vulkan | OpenCL");
            println!("  -------------|------|-----|-----|--------|--------");
            println!("  Locked       |  ❌  |  ❌ |  ❌ |   ❌   |   ❌  ");
            println!("  Basic        |  ✅  |  ❌ |  ❌ |   ❌   |   ❌  ");
            println!("  Standard     |  ✅  |  ✅ |  ✅ |   ❌   |   ❌  ");
            println!("  Advanced     |  ✅  |  ✅ |  ✅ |   ✅   |   ❌  ");
            println!("  GodMode      |  ✅  |  ✅ |  ✅ |   ✅   |   ✅  ");
            println!();

            let active_modes = poder.capabilities.supported_modes.len();
            println!("═══════════════════════════════════════════════════════════════════");
            println!("🏆 SUA SITUAÇÃO: {} modo(s) disponível(is)", active_modes);
            println!("═══════════════════════════════════════════════════════════════════");

            match active_modes {
                0 => println!("\n  🚫 SEM ACESSO - Nenhum recurso GPU disponível\n"),
                1 => println!("\n  🔓 ACESSO BÁSICO - Apenas Leitura\n"),
                2 => println!("\n  📊 ACESSO PADRÃO - Monitoramento\n"),
                3 | 4 => println!("\n  ⚡ ACESSO AVANÇADO - Compute\n"),
                5 => println!("\n  🌟 GOD MODE - ACESSO TOTAL\n"),
                _ => {}
            }

            if active_modes < 5 {
                println!("  Para desbloquear mais modos:");
                if !poder.capabilities.has_opencl {
                    println!("    • Instalar OpenCL (pocl ou driver proprietário)");
                }
                if !poder.capabilities.has_vulkan {
                    println!("    • Instalar driver Vulkan (Turnip/Freedreno)");
                }
                println!();
            }

            println!("═══════════════════════════════════════════════════════════════════");
            println!("📋 JSON STATUS");
            println!("═══════════════════════════════════════════════════════════════════");
            let status = poder.get_status();
            println!(
                "{}\n",
                serde_json::to_string_pretty(&status).unwrap_or_default()
            );
        }
        Err(e) => {
            error!("Falha ao inicializar: {}", e);
            println!("\n❌ ERRO: {}\n", e);
            std::process::exit(1);
        }
    }
}
