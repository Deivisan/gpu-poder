# DSP/ADSP para Computação ML - Análise Técnica

## O que são os DSPs no Snapdragon 695

O Snapdragon 695 possui **dois processadores de sinal digital (DSPs)**:

### 1. ADSP (Audio DSP)
- **Localização**: `/dev/adsprpc-smd`
- **Função**: Processamento de áudio, voz, Always-on
- **Acessível**: ✅ (princípio de Arquitetura)

### 2. CDSP (Compute DSP)  
- **Localização**: `/dev/subsys_cdsp`
- **Função**: Computação de ML/AI, processamento de imagem
- **Acessível**: Parcialmente (subsystem disponível)

## Hardware Disponível no Seu Dispositivo

```
/dev/
├── adsprpc-smd              # DSP principal (ACESSÍVEL)
├── adsprpc-smd-secure       # DSP seguro
├── ramdump_adsp            # Dump de memória ADSP
├── ramdump_cdsp            # Dump de memória CDSP
├── subsys_adsp             # Subsistema ADSP
└── subsys_cdsp            # Subsistema CDSP
```

## Software Necessário para Usar os DSPs

### Opção 1: Qualcomm FastRPC (Oficial)
- **Repositório**: https://github.com/quic/fastrpc
- **Wrapper Linux**: https://github.com/linux-msm/hexagonrpc
- **Bibliotecas**: Precisam ser copiadas do Android/Telefone

### Opção 2: TensorFlow Lite Hexagon Delegate
- **Status**: Experimental (descontinuado em favor de NNAPI)
- **Suporte**: Hexagon 680, 682, 685, 690
- **Snapdragon 695**: Hexagon 692 (não oficialmente suportado)

### Opção 3: SNPE (Snapdragon Neural Processing Engine)
- **Status**: Requer registro de desenvolvedor Qualcomm
- **Plataformas**: Ubuntu 18.04/20.04 (não Arch ARM)
- **DSP**: Suporta Hexagon DSP para inference

## Limitação Principal

Para usar os DSPs, você precisa de:

1. **Firmware do DSP** - Binários específicos para o SoC
   - Estão em `/lib/firmware/qcom/` no Android
   - Precisam ser copiados para o Arch chroot

2. **Bibliotecas userspace** - `libadsprpc.so`, `libhexagon_nom_skel.so`, etc.
   - Disponíveis no Android em `/vendor/lib64/`
   - Precisam ser copiadas

3. **Acesso root** - Para carregar firmware

## Alternativas Práticas para Seu Caso

### 1. Usar CPU Otimizada (Já Funcional)
O whisper.cpp já usa NEON/FP16:
- **Resultado atual**: 0.30x realtime (3x mais rápido que tempo real)
- **Sem necessidade de drivers adicionais**

### 2. Vulkan Compute (Parcialmente Funcional)
- Driver Vulkan instalado ✅
- Bibliotecas compiladas ✅
- Precisa debinário funcional

### 3. Future: Compilar FastRPC
Se quiser continuar no futuro:
```bash
# Precisaria de:
# 1. Firmware do Android (/lib/firmware/qcom/)
# 2. Bibliotecas libadsprpc.so do Android
# 3. Build com meson/ninja
git clone https://github.com/linux-msm/hexagonrpc
cd hexagonrpc
meson setup build
ninja -C build
```

## Conclusão

| Abordagem | Status | Esforço |
|-----------|--------|---------|
| CPU (whisper.cpp) | ✅ Funcionando | Zero |
| Vulkan GPU | ⚠️ Parcial | Médio |
| DSP/ADSP | ❌ Requer Android | Alto |

**Recomendação**: O setup atual com CPU já é funcional para transcrição de áudio. O DSP requereria extrair bibliotecas do Android e não é trivial em um chroot.

---
*Atualizado: 24/02/2026*
