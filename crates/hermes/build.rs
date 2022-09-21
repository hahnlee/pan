fn main() {
    let dst = cmake::Config::new("binding")
        .generator("Ninja")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=binding");
}
