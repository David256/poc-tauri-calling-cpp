// src/build.rs
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/main.cpp")
        .compile("main.a");
}
