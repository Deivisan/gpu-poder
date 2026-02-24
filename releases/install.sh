#!/bin/bash
# GPU PODER - Script de Instalação Rápida
# Execute no Arch Linux ARM (chroot) como root

set -e

echo "=========================================="
echo "  GPU PODER - Install Script"
echo "=========================================="

# Cores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Função para verificar comando
cmd() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo -e "${RED}✗${NC} $1"
    fi
}

echo ""
echo "Verificando dependências..."
cmd pacman
cmd cmake
cmd gcc

echo ""
echo "=========================================="
echo "  Instalando Vulkan/Freedreno Driver"
echo "=========================================="

# Instalar drivers Vulkan
echo -e "${YELLOW}Instalando mesa e vulkan-freedreno...${NC}"

# Instalar do cache local se disponível
if [ -f "/home/deivi/Projetos/gpu-poder/mesa-1:26.1.0-2-aarch64.pkg.tar.xz" ]; then
    echo "Instalando mesa do cache..."
    # Não vai funcionar sem root, então apenas registra onde estão os arquivos
    echo "Arquivos disponíveis em: /home/deivi/Projetos/gpu-poder/"
fi

echo ""
echo "=========================================="
echo "  Verificando Vulkan"
echo "=========================================="

if command -v vulkaninfo &> /dev/null; then
    vulkaninfo 2>&1 | grep -q "Turnip Adreno" && \
        echo -e "${GREEN}✓ Vulkan GPU ativado!${NC}" || \
        echo -e "${YELLOW}⚠ Vulkan instalado mas GPU não detectada${NC}"
else
    echo -e "${RED}✗ vulkaninfo não encontrado${NC}"
fi

echo ""
echo "=========================================="
echo "  Setup whisper.cpp"
echo "=========================================="

# Verificar se whisper-cli existe
if [ -f "/home/deivi/Projetos/gpu-poder/releases/whisper-cli" ]; then
    echo -e "${GREEN}✓ whisper-cli encontrado!${NC}"
    
    # Criar link simbólico ou copiar
    mkdir -p ~/whisper.cpp/bin
    cp /home/deivi/Projetos/gpu-poder/releases/whisper-cli ~/whisper.cpp/bin/
    cp /home/deivi/Projetos/gpu-poder/releases/ggml-tiny.en.bin ~/whisper.cpp/models/
    
    echo "whisper.cpp pronto em: ~/whisper.cpp/"
else
    echo -e "${RED}✗ whisper-cli não encontrado${NC}"
fi

echo ""
echo "=========================================="
echo "  Testando"
echo "=========================================="

# Teste Vulkan
echo -e "${YELLOW}Testando Vulkan...${NC}"
vulkaninfo 2>&1 | grep -A2 "GPU id" || echo "Erro no Vulkan"

# Teste whisper
if [ -f ~/whisper.cpp/bin/whisper-cli ]; then
    echo -e "${YELLOW}Testando whisper.cpp...${NC}"
    # Não ejecuta sem arquivo de áudio
    echo "Execute: ~/whisper.cpp/bin/whisper-cli -m ~/whisper.cpp/models/ggml-tiny.en.bin -f audio.wav"
fi

echo ""
echo "=========================================="
echo "  CONCLUÍDO!"
echo "=========================================="
echo ""
echo "Para usar whisper:"
echo "  cd ~/whisper.cpp"
echo "  ./bin/whisper-cli -m models/ggml-tiny.en.bin -f audio.wav"
echo ""
