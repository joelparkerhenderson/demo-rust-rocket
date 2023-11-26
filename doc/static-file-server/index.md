# Static file server

Rocket can serve static files via the FileServer custom handler.

See source:
<../../src/static_file_server.rs>

See Rocket API:
<https://api.rocket.rs/v0.5/rocket/fs/struct.FileServer.html>


## Create directory and files

Create any directory and any files, such as:

```sh
mkdir -p www/static-files-with-path-segments
cd $_
echo "Demo 1 of static files with a file server" > demo1.txt
echo "Demo 2 of static files with a file server" > demo2.txt
```


## Add code

Edit `main.rs` and use the class:

```rust
use rocket::fs::FileServer;
```

Edit the `main`` function and mount the route, such as:

```rust
.mount("/static-files-with-a-file-server", FileServer::from(...))
```


## Choose a path

The `FileServer::from` function uses an absolute path to a system directory.

One way to configure the file server directory path is to create a shell environment variable `FILES_DIR` to point to a system directory path, such as:

```sh
export FILES_DIR="/a/b/c"
```

The mount code is:

```rust
.mount("/static-files-with-a-file-server", FileServer::from(env!("FILES_DIR")))
```

Another way to configure the file server directory path is to use a relative path
based on the program's cargo manifest directory; this enables the program to get
the directory path via a Rust environment variable that is automatically set by
cargo. For example, this demo puts the code in the relative directory `www/files/`.

The mount code is:

```rust
.mount(
    "/static-files-with-a-file-server", 
    FileServer::from(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www")
        .join("static-files-with-a-file-server")
    )
)
```
