fn main() {
    // Compile the C code
    cc::Build::new()
        .file("foreign-code/calculator.c")
        .compile("calculator");
    
    println!("cargo:rerun-if-changed=foreign-code/calculator.c");
}
