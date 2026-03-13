# Napkin Runbook - gpu-poder

## Curation Rules
- Re-prioritize on every read.
- Keep recurring, high-value notes only.
- Max 10 items per category.
- Each item includes date + "Do instead".

## Execution & Validation (Highest Priority)
1. **[2026-03-13] Driver Turnip/Mesa para Adreno 619**
   Do instead: Compilar no PC Desktop, transferir binários via Tailscale.

2. **[2026-03-13] Repo muito grande (1.5GB)**
   Do instead: NÃO compilar no mobile, usar file server para builds.

3. **[2026-03-13] Vulkan 1.4 target**
   Do instead: Testar com vkcube, validar compute shaders.

## Shell & Command Reliability
1. **[2026-03-13] Mesa build requer deps pesadas**
   Do instead: Meson + Ninja no PC, cross-compile para ARM64.

2. **[2026-03-13] Turnip driver específico Adreno**
   Do instead: Usar freedreno/turnip branch, não generic Mesa.

## Domain Behavior Guardrails
1. **[2026-03-13] GPU compute only (sem display)**
   Do instead: Foco em compute shaders, não rendering tradicional.

2. **[2026-03-13] Thermal crítico em GPU tasks**
   Do instead: Monitorar /sys/class/kgsl/kgsl-3d0/temp durante testes.

## User Directives
1. **[2026-03-13] Public repo para comunidade**
   Do instead: Documentar processo de build, compartilhar binários.

2. **[2026-03-13] Integração com Remotion**
   Do instead: Usar ANGLE/Vulkan para video rendering headless.
