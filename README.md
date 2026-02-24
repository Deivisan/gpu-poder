# GPU PODER - Projeto de Aceleração GPU no Poco X5 5G

## 📱 Hardware

| Componente | Especificação |
|------------|---------------|
| **Dispositivo** | Poco X5 5G |
| **SoC** | Snapdragon 695 5G |
| **GPU** | Adreno 619v2 |
| **Arquitetura** | ARM64 (aarch64) |
| **RAM** | 8GB (7.3GB disponível) |
| **Sistema** | Arch Linux ARM (chroot no Termux) |

---

## 🎯 Status Atual

```
╔═══════════════════════════════════════════════════════════════╗
║                     ACELERAÇÃO GPU                            ║
╠═══════════════════════════════════════════════════════════════╣
║  Vulkan Driver (Turnip)    ✅ FUNCIONANDO                    ║
║  Compute Shaders           ✅ 2048 workgroup invocations      ║
║  KGSL GPU Access          ✅ Leitura/escrita                 ║
║  DSP/ADSP                 ⚠️  Dispositivos detectados        ║
║  whisper.cpp               ✅ FUNCIONANDO (CPU)              ║
║  faster-whisper (Python)   ✅ FUNCIONANDO                    ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## ✅ O que Funciona

### 1. Vulkan GPU (Turnip Adreno 619)

```bash
$ vulkaninfo | grep -A5 "GPU id"

GPU id = 0 (Turnip Adreno (TM) 619)
deviceName = Turnip Adreno (TM) 619
apiVersion = 1.3.296
driverVersion = 24.3.0
```

**Capabilities:**
- `maxComputeWorkGroupInvocations`: 2048
- `maxComputeSharedMemorySize`: 32768 bytes
- `apiVersion`: Vulkan 1.3.296

### 2. KGSL (Kernel Graphics Support Layer)

```bash
$ cat /sys/class/kgsl/kgsl-3d0/clock_mhz
266

$ cat /sys/class/kgsl/kgsl-3d0/freq_table_mhz
840 770 650 490 390 266
```

### 3. whisper.cpp (Transcrição de Áudio)

```bash
$ cd ~/whisper.cpp
$ ./build/bin/whisper-cli -m models/ggml-tiny.en.bin -f samples/jfk.wav
```

**Resultado:**
```
Audio: 11 segundos
Tempo: 3.07s
Fator tempo real: 0.28x (3.6x mais rápido que o áudio!)
Texto: "and so my fellow Americans ask not what your country can do for you..."
```

### 4. faster-whisper (Python)

```python
from faster_whisper import WhisperModel

model = WhisperModel('tiny', device='cpu', compute_type='int8')
segments, info = model.transcribe('audio.wav')
print(' '.join([s.text for s in segments]))
```

**Resultado:**
```
Audio: 11 segundos
Tempo: 3.36s
Fator tempo real: 0.30x
```

---

## ⚠️ O que Não Funciona (Limitação)

### OpenCL
- **Status**: rusticl instalado, mas sem dispositivos
- **Motivo**: Não existe driver OpenCL pré-compilado para Adreno no Arch ARM
- **Alternativa**: Vulkan está disponível

### DSP/ADSP
- **Status**: Dispositivos detectados (`/dev/adsprpc-smd`)
- **Motivo**: Requer firmwares e bibliotecas do Android
- **Esforço**: Alto (precisa extrair do Android)

### Vulkan nos binários
- **Status**: Bibliotecas compiladas, mas executáveis com problemas de link
- **Motivo**: Símbolos ausentes no build final

---

## 📂 Estrutura do Projeto

```
gpu-poder/
├── src/
│   ├── lib.rs              # API principal
│   ├── core.rs             # Engine GPU
│   ├── modes.rs            # 5 modos de permissão
│   ├── kgsl.rs             # Acesso KGSL
│   ├── kgsl_direct.rs      # KGSL via sysfs
│   ├── drm.rs              # DRM/KMS
│   ├── dsp.rs              # DSP/ADSP
│   ├── compute.rs          # Vulkan/OpenCL
│   ├── main.rs             # CLI status
│   └── monitor.rs          # Monitor tempo real
├── build/                  # Binários compilados
├── Cargo.toml
├── DSP_ANALISE.md         # Análise de DSPs
├── INVESTIGACAO.md        # Investigação técnica
├── setup-ai-tools.sh      # Script setup AI
└── test_whisper.py        # Teste whisper
```

---

## 🧪 Testes Realizados

### Teste 1: Vulkan Info
```bash
$ vulkaninfo | grep "Turnip Adreno"
GPU id = 0 (Turnip Adreno (TM) 619)
```
**Resultado**: ✅ SUCESSO

### Teste 2: whisper.cpp tiny.en
```bash
$ ./build/bin/whisper-cli -m models/ggml-tiny.en.bin -f samples/jfk.wav
```
**Resultado**: ✅ Transcreveu em 3.07s (0.28x realtime)

### Teste 3: faster-whisper Python
```python
model = WhisperModel('tiny', device='cpu', compute_type='int8')
```
**Resultado**: ✅ Funciona com int8, float16 dá erro

### Teste 4: KGSL sysfs
```bash
$ cat /sys/class/kgsl/kgsl-3d0/gpubusy
```
**Resultado**: ✅ Leitura OK

### Teste 5: DSP devices
```bash
$ ls /dev/adsprpc* /dev/subsys_*
```
**Resultado**: ⚠️ Dispositivos existem mas requerem firmware

---

## 🚀 Como Usar

### Transcrição de Áudio (whisper.cpp)

```bash
# Entrar no diretório
cd ~/whisper.cpp

# Transcrever áudio
./build/bin/whisper-cli -m models/ggml-tiny.en.bin -f seu_audio.wav

# Com mais threads
./build/bin/whisper-cli -m models/ggml-tiny.en.bin -f seu_audio.wav -t 8
```

### Transcrição com Python (faster-whisper)

```python
from faster_whisper import WhisperModel

# Usar modelo tiny (mais rápido)
model = WhisperModel('tiny', device='cpu', compute_type='int8')

# Transcrever
segments, info = model.transcribe('audio.wav')

# Resultado
for segment in segments:
    print(segment.text)
```

### Monitoramento GPU

```bash
# Verificar Vulkan
vulkaninfo | grep "Turnip Adreno"

# Verificar KGSL
cat /sys/class/kgsl/kgsl-3d0/clock_mhz
cat /sys/class/kgsl/kgsl-3d0/gpubusy

# Verificar temperatura
cat /sys/class/thermal/thermal_zone*/temp
```

---

## 📊 Performance

| Ferramenta | Tipo | Tempo (11s áudio) | Realtime Factor |
|------------|------|-------------------|-----------------|
| whisper.cpp (tiny) | CPU | 3.07s | 0.28x |
| faster-whisper (int8) | CPU | 3.36s | 0.30x |
| whisper.cpp (small) | CPU | ~8s | 0.73x |

---

## 🔧 Comandos Úteis

```bash
# Vulkan info
vulkaninfo 2>&1 | grep -A10 "GPU"

# GPU status
cat /sys/class/kgsl/kgsl-3d0/clock_mhz
cat /sys/class/kgsl/kgsl-3d0/gpubusy

# DSP devices
ls -la /dev/adsprpc* /dev/subsys_*

# Whisper transcription
~/whisper.cpp/build/bin/whisper-cli -m ~/whisper.cpp/models/ggml-tiny.en.bin -f audio.wav
```

---

## 📌 Notas Importantes

1. **whisper.cpp funciona em CPU** com aceleração NEON/FP16
2. **Vulkan driver está instalado** mas os binários com GPU enabled têm problemas de link
3. **DSP requer firmwares** do Android (alto esforço)
4. **compute_type=int8** é o mais estável no faster-whisper

---

## 🔄 Próximos Passos (Opcional)

1. **OpenCL**: Requer driver específico para Adreno (não disponível)
2. **DSP/ADSP**: Requer extrair bibliotecas do Android
3. **Vulkan executáveis**: Problema de linker nas bibliotecas GGML

---

**Última atualização**: 24/02/2026
**Projeto**: gpu-poder - Aceleração GPU no Poco X5 5G
