# Target help


Rust not having a runtime means that it doesn't have a lot of code running as part of the language (for example a garbage collector or bytecode interpreter). It does still need to use operating system primitives (i.e. syscalls), and these are different on MacOS and Linux.

What you want is a cross compiler. If you're using rustup, then installing a cross compiler should be simple.

Target examples:

* x86_64-unknown-linux-gnu such as Debian, Ubuntu, RedHat, Fedora, CentOS.

* x86_64-unknown-linux-musl such as Alpine.


## Target to Linux GNU

```
rustup target add x86_64-unknown-linux-gnu
```

Then building is:

```
cargo build --release --target=x86_64-unknown-linux-gnu
```


## Target from macOS to Linux GNU

macOS needs a cross-platform compiler, such as GCC or MUSL cross:

```
brew install gcc
```

...or...

```
brew install FiloSottile/musl-cross/musl-cross
```

Then adjust `.cargo/config` to use the compiler:

```
edit .cargo/config
```

Example configuration:

```
[target.x86_64-unknown-linux-gnu]
linker = "gcc"
runner = "my-emulator"
rustflags = ["…", "…"]


## Target from macOS to Linux MUSL

Edit `.cargo/config` to configure:

```
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

This should instruct Rust, whenever the target is set to --target=x86_64-unknown-linux-musl, to use the executable `x86_64-linux-musl-gcc` to link the compiled objects. But it seems to be the case that if you have any C code compiled by a Rust build script, you also have to set environment variables like TARGET_CC to get it working. So when my code started throwing linking errors, I just ran the following in my shell:

```
export TARGET_CC=x86_64-linux-musl-gcc
```
