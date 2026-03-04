fn main() {
    // Link against libdrm
    println!("cargo:rustc-link-lib=drm");
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/usr/lib/aarch64-linux-gnu");
    
    // Try pkg-config
    if pkg_config::probe_library("libdrm").is_ok() {
        return;
    }
    
    // Fallback: manual linking
    println!("cargo:rustc-link-lib=drm_amdgpu");
}
