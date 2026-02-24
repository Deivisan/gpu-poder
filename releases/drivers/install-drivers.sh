#!/bin/bash
# GPU PODER - Driver Installation Script
# Execute como ROOT após formatar o celular

set -e

echo "=========================================="
echo "  GPU PODER - Driver Installer"
echo "=========================================="

DRIVER_DIR="$(cd "$(dirname "$0")" && pwd)/drivers"

echo "Driver directory: $DRIVER_DIR"

# Cores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Verificar root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}Execute como root!${NC}"
    exit 1
fi

echo ""
echo "=========================================="
echo "  Instalando Turnip/Freedreno Driver"
echo "=========================================="

# Copiar bibliotecas
echo "Copiando bibliotecas..."
cp -v "$DRIVER_DIR/libvulkan_freedreno.so" /usr/lib/
cp -v "$DRIVER_DIR/libdrm_freedreno.so"* /usr/lib/
cp -v "$DRIVER_DIR/libstdc++.so.6"* /usr/lib/

# Copiar ICD configs
echo "Copiando configurações ICD..."
mkdir -p /usr/share/vulkan/icd.d/
cp -v "$DRIVER_DIR/vulkan_icd.d/"*.json /usr/share/vulkan/icd.d/

# Atualizar ldconfig
echo "Atualizando cache de bibliotecas..."
ldconfig

echo ""
echo "=========================================="
echo "  Verificando Instalação"
echo "=========================================="

# Testar vulkaninfo
if command -v vulkaninfo &> /dev/null; then
    if vulkaninfo 2>&1 | grep -q "Turnip Adreno"; then
        echo -e "${GREEN}✓ Vulkan Turnip Adreno instalado!${NC}"
        vulkaninfo 2>&1 | grep "GPU id"
    else
        echo -e "${YELLOW}⚠ Vulkan instalado mas GPU não detectada${NC}"
    fi
else
    echo -e "${RED}✗ vulkaninfo não encontrado${NC}"
fi

echo ""
echo "=========================================="
echo "  CONCLUÍDO!"
echo "=========================================="
echo ""
echo "Driver Vulkan instalado com sucesso!"
echo ""
