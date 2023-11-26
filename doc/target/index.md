# Target help


Rust not having a runtime means that it doesn't have a lot of code running as part of the language (for example a garbage collector or bytecode interpreter). It does still need to use operating system primitives (i.e. syscalls), and these are different on MacOS and Linux.

What you want is a cross compiler. If you're using rustup, then installing a cross compiler should be simple.

Target examples:

* x86_64-unknown-linux-gnu such as Debian, Ubuntu, RedHat, Fedora, CentOS.

* x86_64-unknown-linux-musl such as Alpine.


## Target to Linux GNU

Add target:

```sh
rustup target add x86_64-unknown-linux-gnu
```

Build release:

```sh
cargo build --release --target=x86_64-unknown-linux-gnu
```


## Target to Linux MUSL

Add target:

```sh
rustup target add x86_64-unknown-linux-musl
```

Build release:

```sh
cargo build --release --target=x86_64-unknown-linuux-musl
```



## Target from macOS to Linux GNU via GCC

Add:

```
rustup target add x86_64-unknown-linux-gnu
```

Install cross-platform compiler:

```sh
brew install gcc
```

Install linker:

```sh
brew tap SergioBenitez/osxct
```

Edit `.cargo/config` to use it:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```

Env:

```sh
# Linker for the target platform
# (cc can also be updated using .cargo/config)
export TARGET_CC="x86_64-unknown-linux-gnu-gcc"

# Library headers to link against
export TARGET_CFLAGS="-I $(pwd)/usr/include/x86_64-linux-gnu -isystem $(pwd)/usr/include"

# Libraries (shared objects) to link against
export LD_LIBRARY_PATH="$(pwd)/usr/lib/x86_64-linux-gnu;$(pwd)/lib/x86_64-linux-gnu"

# openssl-sys specific build flags
export OPENSSL_DIR="$(pwd)/usr/"
export OPENSSL_LIB_DIR="$(pwd)/usr/lib/x86_64-linux-gnu/"
```

Build:

```sh
RUST_BACKTRACE=1 \
AS=x86_64-unknown-linux-as \
PKG_CONFIG_ALLOW_CROSS=1 \
TARGET_CC=clang \
CFLAGS="-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include" \
cargo build --target=x86_64-unknown-linux-gnu --release
```


## Target from macOS to Linux MUSL via GCC

Install a Linux MUSL target:

```
rustup target add x86_64-unknown-linux-musl
```

Install cross compiler:

```sh
brew install FiloSottile/musl-cross/musl-cross
```

Edit `.cargo/config` to use the compiler:

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

Set env:

```sh
# Linker for the target platform
# (cc can also be updated using .cargo/config)
export TARGET_CC="x86_64-unknown-linux-musl-gcc"

# Library headers to link against
export TARGET_CFLAGS="-I $(pwd)/usr/include/x86_64-linux-musl -isystem $(pwd)/usr/include"

# Libraries (shared objects) to link against
export LD_LIBRARY_PATH="$(pwd)/usr/lib/x86_64-linux-musl;$(pwd)/lib/x86_64-linux-musl"

# openssl-sys specific build flags
export OPENSSL_DIR="$(pwd)/usr/"
export OPENSSL_LIB_DIR="$(pwd)/usr/lib/x86_64-linux-musl/"
```

Build:

```sh
RUST_BACKTRACE=1 \
AS=x86_64-unknown-linux-as \
PKG_CONFIG_ALLOW_CROSS=1 \
TARGET_CC=clang \
CFLAGS="-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include" \
cargo build --target=x86_64-unknown-linux-musl --release
```
