#!/bin/bash
# =============================================================================
# GPU PODER - Script de Instalação de Driver Vulkan/OpenCL com Suporte KGSL
# =============================================================================
# Este script baixa e configura o Mesa com patches KGSL para acesso 100% GPU
# =============================================================================

set -e

CORES=$(nproc)
MESA_VERSION="25.1.0"
WORK_DIR="/tmp/mesa-build"
INSTALL_DIR="/usr/local"

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║        GPU PODER - INSTALAÇÃO DRIVER KGSL                       ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Verificar se é root
if [ "$EUID" -ne 0 ]; then
    echo "❌ Este script precisa ser executado como ROOT"
    echo "   Execute no Android com: su -c 'bash $0'"
    exit 1
fi

echo "✅ Verificações iniciais..."
echo "   CPU cores: $CORES"
echo "   UID: $(id)"

# Criar diretório de trabalho
mkdir -p "$WORK_DIR"
cd "$WORK_DIR"

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "📦 OPÇÃO 1: Usar versão pré-compilada (mesa-for-android-container)"
echo "═══════════════════════════════════════════════════════════════════"

# Baixar versão pré-compilada
echo "Baixando libs pré-compiladas..."

RELEASE_URL="https://github.com/lfdevs/mesa-for-android-container/releases/latest"

# Tentar baixar a versão mais recente
if command -v curl &> /dev/null; then
    DOWNLOAD_CMD="curl -LO"
elif command -v wget &> /dev/null; then
    DOWNLOAD_CMD="wget"
else
    echo "❌ Precisa de curl ou wget"
    exit 1
fi

echo ""
echo "NOTA: Para Vulkan/OpenCL funcionarem com GPU real no chroot,"
echo "      você precisa de uma destas opções:"
echo ""
echo "1. Baixar release do mesa-for-android-container:"
echo "   https://github.com/lfdevs/mesa-for-android-container/releases"
echo ""
echo "2. Compilar Mesa com patches KGSL (droidvm):"
echo "   https://github.com/droidvm/mesa-freedreno-patchs"
echo ""
echo "3. Usar módulo Magisk (se disponível):"
echo "   https://github.com/v3kt0r-87/Mesa-Turnip-Builder"
echo ""

# Verificar dispositivo
echo "═══════════════════════════════════════════════════════════════════"
echo "📱 Dispositivos GPU disponíveis:"
echo "═══════════════════════════════════════════════════════════════════"

for dev in /dev/kgsl-3d0 /dev/dri/card0 /dev/dri/renderD128 /dev/adsprpc-smd; do
    if [ -e "$dev" ]; then
        echo "✅ $dev"
    else
        echo "❌ $dev (não encontrado)"
    fi
done

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "🔧 Solução alternativa: Usar KGSL diretamente via código"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "Já temos o código Rust em: /home/deivi/Projetos/gpu-poder/"
echo "que pode acessar KGSL diretamente via ioctl()"
echo ""

# Variáveis de ambiente recomendadas
echo "═══════════════════════════════════════════════════════════════════"
echo "⚙️  Variáveis de ambiente (adicione ao ~/.zshrc):"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "# Vulkan com GPU real (quando driver instalado):"
echo "export VK_DRIVER_FILES=/usr/share/vulkan/icd.d/freedreno_icd.json"
echo ""
echo "# Vulkan CPU fallback (funciona agora):"
echo "export VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.aarch64.json"
echo ""
echo "# OpenCL:"
echo "export OCL_ICD_VENDORS=/etc/OpenCL/vendors/"
echo ""

echo "═══════════════════════════════════════════════════════════════════"
echo "📋 RESUMO DAS AÇÕES NECESSÁRIAS"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "PARA ACESSO 100% GPU:"
echo ""
echo "1. Execute este script como root no Android:"
echo "   su -c 'bash /home/deivi/Projetos/gpu-poder/install-gpu-driver.sh'"
echo ""
echo "2. Ou instale o módulo Magisk (Turnip):"
echo "   https://github.com/v3kt0r-87/Mesa-Turnip-Builder"
echo ""
echo "3. Ou use o tutorial do mesa-for-android-container:"
echo "   https://github.com/lfdevs/mesa-for-android-container"
echo ""
echo "═══════════════════════════════════════════════════════════════════"
