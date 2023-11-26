# Static file

See source:
[src/demos/static_file.rs](../../src/demos/static_file.rs)

See Rocket API:
<https://api.rocket.rs/v0.5/rocket/fs/struct.NamedFile.html>.


## Create directory and file

Create any directory and any file, such as:

```sh
mkdir -p www/static-file
cd $_
echo "Demo of a static file" > demo.html
```


## Add code

Relevant handler:

```rust
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/static-file")]
pub async fn get() -> Option<NamedFile> {
    NamedFile::open(
        Path::new("www/static-file/demo.html")
    ).await.ok()
}
```
