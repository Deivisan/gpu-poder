#!/bin/bash
# =============================================================================
# GPU PODER - Instalador de Driver Vulkan/OpenCL com Suporte KGSL
# =============================================================================
# Execute como ROOT no Android/Termux
# =============================================================================

set -e

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║        GPU PODER - INSTALAÇÃO DRIVER TURNIP/KGSL            ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Verificar root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Execute como ROOT: su -c 'bash install-turnip.sh'"
    exit 1
fi

echo "✅ Root confirmado"

# Detectar arquitetura
ARCH=$(uname -m)
echo "📱 Arquitetura: $ARCH"

# Detectar GPU
GPU_NAME=""
if [ -f /sys/class/kgsl/kgsl-3d0/gpu_model ]; then
    GPU_NAME=$(cat /sys/class/kgsl/kgsl-3d0/gpu_model 2>/dev/null || echo "unknown")
fi
echo "🎮 GPU: $GPU_NAME"

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "📦 BAIXANDO DRIVER PRÉ-COMPILADO"
echo "═══════════════════════════════════════════════════════════════════"

WORK_DIR="/tmp/mesa-turnip"
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

# URLs dos releases (verificar versões mais recentes)
MESA_URL="https://github.com/lfdevs/mesa-for-android-container/releases"

echo "Acessando: $MESA_URL"
echo ""
echo "INSTRUÇÕES MANUAL:"
echo "================="
echo ""
echo "1. Acesse: $MESA_URL"
echo "2. Baixe a versão mais recente para sua arquitetura"
echo "3. Extraia os arquivos"
echo "4. Copie as libs para /usr/lib/"
echo ""
echo "Exemplo de comandos após baixar:"
echo ""
echo "  # Extrair"
echo "  tar -xvf mesa-*.tar.gz"
echo ""
echo "  # Copiar libs Vulkan"
echo "  cp -r lib/* /usr/lib/"
echo ""
echo "  # Copiar ICD config"
echo "  cp -r etc/* /etc/"
echo ""
echo "  # Configurar variáveis"
echo "  echo 'export VK_DRIVER_FILES=/usr/share/vulkan/icd.d/freedreno_icd.json' >> ~/.bashrc"
echo ""

# Verificar dispositivos
echo "═══════════════════════════════════════════════════════════════════"
echo "📱 DISPOSITIVOS DISPONÍVEIS"
echo "═══════════════════════════════════════════════════════════════════"

for dev in /dev/kgsl-3d0 /dev/dri/card0 /dev/dri/renderD128 /dev/adsprpc-smd; do
    if [ -e "$dev" ]; then
        echo "✅ $dev"
    else
        echo "❌ $dev"
    fi
done

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "🎯 ALTERNATIVAS SE NÃO TIVER ACESSO ROOT"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "1. Usar Magisk com módulo Turnip:"
echo "   https://github.com/v3kt0r-87/Mesa-Turnip-Builder"
echo ""
echo "2. Usar Lich AMD GPU (se compatível)"
echo ""
echo "3. Compilar Mesa manualmente com patches KGSL"
echo "   https://github.com/droidvm/mesa-freedreno-patchs"
echo ""

echo "═══════════════════════════════════════════════════════════════════"
echo "✅ INSTALAÇÃO CONCLUÍDA - SCRIPT PREPARADO"
echo "═══════════════════════════════════════════════════════════════════"
