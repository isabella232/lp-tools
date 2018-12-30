fn main() {
    println!("cargo:rustc-link-lib=tag");

    gcc::Build::new()
        .cpp(true)
        .file("src/cpp/taglib.cpp")
        .compile("libtaglib.a")
}
