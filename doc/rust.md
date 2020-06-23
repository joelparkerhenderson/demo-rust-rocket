# Rust notes


## Install Rust nightly

Install Rust nightly on Ubuntu in the user's directory:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh
export PATH="$HOME/.cargo/bin:${PATH}"
rustup default nightly
```


## Install Shared Compilation Cache

If you want to use the tool `sccache` for caching your Rust builds, then you can configure it for your user.

Find the path to your `sscache` command, such as:

```sh
$ which sccache
/usr/local/bin/sccache
```

Edit `$HOME/.cargo/config` and configure it:

```toml
[build]
rustc-wrapper = "/usr/local/bin/sccache"
```

The compiler will do parallel jobs, and default to the number of jobs just under
what the compiler believes the CPU can support. We prefer to cap the jobs to 1 at a time, by default, because this helps our systems continue to be responsive for other work, and also helps solve a known issue where `sscache` runs out of time because of too high a simultaneous workload.

Edit `$HOME/.cargo/config` and configure it:

```toml
[build]
jobs = 1
rustc-wrapper = "/usr/local/bin/sccache"
```


## Install Diesel CLI and libs

When we install Diesel CLI on a typical system that we'll be using for development or testing, then we prefer to install it for all the typical database features: Postgres, MySQL, SQLite.

When we install Diesel CLI for a production system that needs to be slim, we prefer to install it for Postgres and skip MySQL and SQLite.

Install dependencies on Debian:

```sh
sudo apt-get install -y libpq-dev
sudo apt-get install -y libmysqlclient-dev
sudo apt-get install -y libsqlite3-dev
```

Install databases:

```sh
sudo apt-get install -y postgresql-client-12
sudo apt-get install -y postgresql-12
sudo apt-get install -y mysql-client
sudo apt-get install -y mysql-server
sudo apt-get install -y sqlite
```

Install:

```sh
cargo install diesel_cli
```
