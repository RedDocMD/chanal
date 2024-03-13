fn main() {
    let raylib = pkg_config::probe_library("raylib").unwrap();
    for link_path in &raylib.link_paths {
        println!("cargo:rustc-link-search={}", link_path.display());
    }
    for lib in &raylib.libs {
        println!("cargo:rust-link-lib={}", lib);
    }
}
