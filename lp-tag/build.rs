fn main() {
    println!("cargo:rustc-link-lib=tag");

    cc::Build::new()
        .cpp(true)
        .file("src/cpp/taglib.cpp")
        .compile("libtaglib.a");
}
