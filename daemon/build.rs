fn main() {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=x264");
    println!("cargo:rustc-link-lib=avresample");
    println!("cargo:rustc-link-lib=postproc");
    println!("cargo:rustc-link-lib=v4l2");
}
