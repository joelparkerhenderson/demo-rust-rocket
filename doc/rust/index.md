# Rust notes


## Get Rust

### For Rocket 0.5 get Rust stable

```sh
rustup default stable
```

### For Rocket 0.4 get Rust nightly

Install Rust nightly in the user's directory:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh
export PATH="$HOME/.cargo/bin:${PATH}"
rustup default nightly
```
