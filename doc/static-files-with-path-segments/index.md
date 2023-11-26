# Static files with path segments

See source:
[src/demos/static_files_with_path_segments.rs](../../src/demos/static_files_with_path_segments.rs)


## Create directory and files

Create any directory and any files, such as:

```sh
mkdir -p www/static-files-with-path-segments
cd $_
echo "Demo 1 of static files with path segments" > demo1.html
echo "Demo 2 of static files with path segments" > demo2.html
```


## Add code

Relevant handler:

```rust
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/static-file")]
pub async fn favicon() -> Option<NamedFile> {
pub async fn pages(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www")
        .join("static-files-with-path-segments")
        .join(path)
    ).await.ok()
}
```
