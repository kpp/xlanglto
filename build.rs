fn main() {
    cc::Build::new()
        .file("mul.c")
        .flag("-flto=thin")
        .compile("libmul.a");
}
