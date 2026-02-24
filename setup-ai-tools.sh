#!/bin/bash
set -e

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║        GPU AI TOOLS - INSTALAÇÃO RÁPIDA                      ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Verificar Vulkan
echo "📊 Verificando Vulkan GPU..."
VULKAN_GPU=$(vulkaninfo 2>/dev/null | grep "deviceName" | head -1 || echo "")
if echo "$VULKAN_GPU" | grep -q "Adreno"; then
    echo "✅ Vulkan GPU: $(echo $VULKAN_GPU | cut -d'=' -f2)"
else
    echo "⚠️ Vulkan pode não estar com GPU real"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "📦 OPÇÕES DE INSTALAÇÃO"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "1. LLAMA.CPP - LLMs com Vulkan GPU"
echo "   https://github.com/ggerganov/llama.cpp"
echo ""
echo "2. WHISPER.CPP - Speech-to-Text com Vulkan"
echo "   https://github.com/ggerganov/whisper.cpp"
echo ""
echo "3. OLLAMA - Interface fácil (precisa compilar)"
echo "   https://github.com/ollama/ollama"
echo ""

# Criar diretório
mkdir -p ~/ai-tools
cd ~/ai-tools

echo "═══════════════════════════════════════════════════════════════════"
echo "📥 Baixando binários pré-compilados (se disponíveis)"
echo "═══════════════════════════════════════════════════════════════════"

# Tentar baixar llama.cpp binário
if [ ! -f llama.cpp ]; then
    echo "Baixando llama.cpp..."
    # Não há binário oficial, precisa compilar
    echo "⚠️ Precisa compilar: cmake + Vulkan"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "🔧 PARA COMPILAR COM VULKAN:"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "# Dependências:"
echo "sudo pacman -S cmake vulkan-headers glslang spirv-tools"
echo ""
echo "# Clone e compile:"
echo "git clone https://github.com/ggerganov/llama.cpp"
echo "cd llama.cpp"
echo "cmake -B build -DCMAKE_VULKAN=on"
echo "cmake --build build --config Release"
echo ""

echo "═══════════════════════════════════════════════════════════════════"
echo "📱 TESTE RÁPIDO"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "# Testar Vulkan compute shader:"
echo "vulkaninfo | grep -i compute"
echo ""

# Testar Vulkan compute
echo "Verificando compute capabilities..."
vulkaninfo 2>/dev/null | grep -i "compute\|maxCompute" | head -5

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "✅ PRÓXIMOS PASSOS"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "1. Compile llama.cpp com Vulkan:"
echo "   cd ~/ai-tools"
echo "   git clone https://github.com/ggerganov/llama.cpp"
echo "   cd llama.cpp"
echo "   cmake -B build -DCMAKE_VULKAN=on"
echo "   cmake --build build"
echo ""
echo "2. Baixe um modelo GGUF:"
echo "   https://huggingface.co/TheBloke"
echo ""
echo "3. Execute com GPU:"
echo "   ./build/bin/llama-server -m modelo.gguf -ngl 100"
echo ""
