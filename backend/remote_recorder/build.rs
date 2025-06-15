#[cfg(target_os = "linux")]
fn main() {
    const PATH: &str = "../../infra/linux/src/pwutils.c";

    let pipewire = pkg_config::Config::new()
        .probe("libpipewire-0.3")
        .expect("PipeWire dev files not found");

    cc::Build::new()
        .file(PATH)
        .includes(pipewire.include_paths)
        .compile("pwutils");

    println!("cargo:rustc-link-lib=pipewire-0.3");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rerun-if-changed={PATH}");
}

#[cfg(target_os = "windows")]
fn main() {}
