# PROMPT PARA COMPILAR DRIVER GPU - TERMUX

---

## 🎯 OBJETIVO

Compilar e instalar driver Vulkan/OpenCL com suporte KGSL para ter acesso 100% à GPU Adreno 619v2 no Arch Linux ARM rodando em chroot dentro do Android (Poco X5 5G).

---

## 📱 HARDWARE

- **Dispositivo:** Poco X5 5G
- **SoC:** Snapdragon 695
- **GPU:** Adreno 619v2
- **CPU:** 8 cores ARM64 (aarch64)
- **RAM:** 7.3 GB

---

## 💻 SOFTWARE

- **Android:** Stock ROM com Magisk (root)
- **Termux:** App terminal no Android
- **Arch Linux ARM:** Chroot dentro do Termux
- **Usuário atual:** `deivi` (uid 1001)
- **Shell:** Zsh + Starship

---

## 🔴 PROBLEMA ATUAL

O Vulkan e OpenCL **INSTALADOS** mas **NÃO FUNCIONAM** com GPU real:

| Recurso | Status | Problema |
|---------|--------|----------|
| Vulkan | Instalado | Driver Freedreno não tem suporte KGSL |
| OpenCL | Instalado | Sem ICD funcional |
| KGSL | Disponível | Acesso leitura apenas |

**Erro Vulkan:** `VK_ERROR_INITIALIZATION_FAILED`
**Causa:** Driver Mesa compilado sem patches KGSL

---

## ✅ O QUE JÁ FUNCIONA

- KGSL sysfs: leitura de clock, temperatura, uso
- DRM: dispositivo existe
- Vulkan via CPU (llvmpipe): funciona mas é lento

---

## 🛠️ ACESSO ROOT

- **Magisk:** Instalado no Android
- **su:** Disponível em `/usr/bin/su`
- **Grupos:** deivi, wheel, kgsl, render, video

---

## 📋 SOLUÇÃO ENCONTRADA

Precisa instalar driver Turnip/Mesa com suporte KGSL:

### Opção 1 (Recomendada):
- **Repo:** https://github.com/lfdevs/mesa-for-android-container
- Versão pré-compilada para containers Android

### Opção 2:
- **Repo:** https://github.com/droidvm/mesa-freedreno-patchs
- Compilar Mesa com patches KGSL

### Opção 3:
- **Repo:** https://github.com/v3kt0r-87/Mesa-Turnip-Builder
- Módulo Magisk

---

## 📁 ARQUIVOS DO PROJETO

Todo o código está em:
```
/home/deivi/Projetos/gpu-poder/
```

Contém:
- Código Rust para acesso GPU
- Scripts de instalação
- Documentação

---

## ⚡ TAREFAS PARA O AGENT

1. **Baixar driver** Turnip/Mesa pré-compilado ou compilar com patches KGSL

2. **Instalar libs** em:
   - `/usr/lib/vulkan/`
   - `/usr/lib/libvulkan_*.so`
   - `/usr/share/vulkan/icd.d/`

3. **Configurar OpenCL** em:
   - `/etc/OpenCL/vendors/`
   - `/usr/lib/libOpenCL.so`

4. **Testar Vulkan:**
```bash
vulkaninfo
```

5. **Testar GPU real:**
```bash
VK_DRIVER_FILES=/usr/share/vulkan/icd.d/freedreno_icd.json vulkaninfo
```

---

## 🔗 LINKS IMPORTANTES

- https://github.com/lfdevs/mesa-for-android-container
- https://github.com/droidvm/mesa-freedreno-patchs
- https://github.com/v3kt0r-87/Mesa-Turnip-Builder
- https://www.reddit.com/r/termux/comments/1nibylk/

---

## 📝 INSTRUÇÕES

Execute como ROOT no Termux:

```bash
su
cd /data/data/com.termux/files/home/Projetos/gpu-poder
bash install-turnip.sh
```

Ou compile manualmente usando os patches do droidvm.

---

**Objetivo final:** Ter Vulkan e OpenCL funcionando com a GPU Adreno 619v2 real, não CPU.

---

**Deivison Santana**
**Setup:** Arch Linux ARM + Android + Magisk + Termux
