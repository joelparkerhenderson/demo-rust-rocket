# OpenSSL notes

The Rust `openssl-sys` crate will automatically detect OpenSSL installations via Homebrew on macOS and vcpkg on Windows. Additionally, it will use pkg-config on Unix-like systems to find the system installation.

Debian and Ubuntu:

```sh
sudo apt-get install pkg-config libssl-dev
```

Fedora and RedHat and CentOS:

```
sudo dnf install pkg-config openssl-devel
```

Arch Linux:

```sh
sudo pacman -S pkg-config openssl
```

macOS:

```sh
brew install openssl@1.1
```


## Missing OpenSSL on macOS

If you need to have openssl@1.1 first in your PATH:

```sh
export PATH="/usr/local/opt/openssl@1.1/bin:$PATH"
```

For compilers to find openssl@1.1 you may need to set:

```sh
export LDFLAGS="-L/usr/local/opt/openssl@1.1/lib"
export CPPFLAGS="-I/usr/local/opt/openssl@1.1/include"
```

For pkg-config to find openssl@1.1 you may need to set:

```sh
export PKG_CONFIG_PATH="/usr/local/opt/openssl@1.1/lib/pkgconfig"
```
