fn main() {
    const PATH: &str = "../../infra/linux/src/pwutils.c";

    cc::Build::new()
        .file(PATH)
        .include("/usr/include/pipewire-0.3")
        .include("/usr/include/spa-0.2")
        .compile("pwutils");

    println!("cargo:rustc-link-lib=pipewire-0.3");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rerun-if-changed={PATH}");
}
