fn main() {
    #[cfg(target_os = "macos")]
    {
	const FRAMEWORKS: [&str; 3] = ["IOKit", "Cocoa", "OpenGL"];
	for framework in FRAMEWORKS {
	    println!("cargo:rustc-link-lib=framework={}", framework);
	}
    }
    let raylib = pkg_config::probe_library("raylib").unwrap();
    for link_path in &raylib.link_paths {
        println!("cargo:rustc-link-search={}", link_path.display());
    }
    for lib in &raylib.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
