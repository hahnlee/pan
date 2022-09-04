fn main() {
    let dst = cmake::Config::new("binding")
        .generator("Ninja")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=binding");
}
