# Install sccache (Shared Compilation Cache)

The tool  `sccache` is a shared compilation cache.

If you want to use `sccache` for caching your Rust builds, then you can configure it for your user.

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

The compiler will do parallel jobs, and default to the number of jobs just under what the compiler believes the CPU can support. We prefer to cap the jobs to 1 at a time, by default, because this helps our systems continue to be responsive for other work, and also helps solve a known issue where `sscache` runs out of time because of too high a simultaneous workload.

Edit `$HOME/.cargo/config` and configure it:

```toml
[build]
jobs = 1
rustc-wrapper = "/usr/local/bin/sccache"
```
