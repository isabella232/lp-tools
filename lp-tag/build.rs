extern crate gcc;

fn main() {
    println!("cargo:rustc-link-lib=tag");

    gcc::Config::new()
        .cpp(true)
        .file("src/cpp/taglib.cpp")
        .compile("libtaglib.a")
}
