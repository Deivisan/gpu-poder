#!/usr/bin/env python3
import sys
import time
from faster_whisper import WhisperModel

def test_whisper(device="auto"):
    print(f"\n{'='*60}")
    print(f"🧪 TESTE FASTER WHISPER - {device.upper()}")
    print(f"{'='*60}")
    
    # Small model for testing
    model_size = "small"
    
    print(f"\n📥 Baixando modelo: {model_size}")
    start = time.time()
    
    try:
        model = WhisperModel(
            model_size,
            device=device,
            compute_type="float16"
        )
        download_time = time.time() - start
        print(f"✅ Modelo baixado em {download_time:.1f}s")
    except Exception as e:
        print(f"❌ Erro ao carregar modelo: {e}")
        return
    
    # Generate dummy audio info
    print(f"\n🎤 Modelo carregado com sucesso!")
    print(f"   Device: {device}")
    print(f"   Compute: float16")
    
    # Info about GPU
    import torch
    print(f"\n📊 PyTorch Info:")
    print(f"   CUDA available: {torch.cuda.is_available()}")
    if torch.cuda.is_available():
        print(f"   GPU: {torch.cuda.get_device_name(0)}")
    
    print(f"\n💡 Para usar GPU real, seria necessário:")
    print(f"   - ONNX Runtime com CUDA/OpenCL")
    print(f"   - ou whisper.cpp com Vulkan (em compilação)")

if __name__ == "__main__":
    device = sys.argv[1] if len(sys.argv) > 1 else "auto"
    test_whisper(device)
