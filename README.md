# About

This project shows how to inline a call to C function from Rust.

# HOWTO

Build with cargo:

```bash
CC=clang-9 RUSTFLAGS="-Clinker-plugin-lto -Clinker=clang-9 -Clink-arg=-fuse-ld=lld-9" cargo build --release
# binary: ./target/release/xlanglto
```

Or build without cargo:

```bash
clang-9 mul.c -flto=thin -c -O2
ar crs libmul.a mul.o
rustc -Clinker-plugin-lto -L. -Copt-level=2 -Clinker=clang-9 -Clink-arg=-fuse-ld=lld-9 src/main.rs -o xlanglto
# binary: ./xlanglto
```

# Result

[Cross-languate LTO](http://blog.llvm.org/2019/09/closing-gap-cross-language-lto-between.html) will result in an inline call to C function:

```bash
$ objdump -d ./target/release/xlanglto | grep "<multiply>:" -A3
000000000002eda0 <multiply>:
   2eda0:	89 f8                	mov    %edi,%eax
   2eda2:	0f af c6             	imul   %esi,%eax
   2eda5:	c3                   	retq
```

# Dependencies

- `clang-9`
- `lld-9`
- `rustc 1.42.0`
