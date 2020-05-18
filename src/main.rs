#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// Rocket makes it easy to serve pages by using "multiple segments".
// You can match against multiple segments by using <param..> in a 
// route path. 
//
// The type of such parameters, known as segments guards, must implement
// FromSegments. A segments guard must be the final component of a path: 
// any text after a segments guard will result in a compile-time error.
//
// Use paths and named files:
//
//     use std::path::PathBuf;
//     use rocket::response::NamedFile;
//
// Create a route and function:
//
//     #[get("/page/<path..>")]
//     fn pages(path: PathBuf) { /* ... */ }
//
use std::path::{Path,PathBuf};
use rocket::response::NamedFile;

#[get("/pages/<path..>")]
fn pages(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/").join(path)).ok()
}

// Rocket makes it easy to serve static files. Use the StaticFiles custom 
// handler from rocket_contrib, which makes it as simple as one line.
//
// Edit Cargo.toml and add:
//
//     rocket_contrib = "*"
//
// Edit main.rs and use the class:
//
//     use rocket_contrib::serve::StaticFiles;
//
// Edit the main fucntion and mount the route:
//
//     rocket.mount("/", StaticFiles::from("/files"))
//
// See https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html
//
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, pages])
    .mount("/", StaticFiles::from("/files"))
    .launch();
}
