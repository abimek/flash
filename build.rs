fn main() {
    println!("cargo:rustc-link-search=native=/usr/lib/llvm-17");
    println!("cargo:rustc-link-lib=LLVM-17");
}
