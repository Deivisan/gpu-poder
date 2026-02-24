cd /home/deivi/Projetos/gpu-poder/build/mesa-24.3.0
for p in ../mesa-freedreno-patchs/freedreno-kgsl-patchs-for-24.3.0/*.patch; do
    echo "Applying ..."
    patch -p1 < 
done
