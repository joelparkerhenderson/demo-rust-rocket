#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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
//     rocket.mount("/public", StaticFiles::from("/static"))
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
    .mount("/", routes![index])
    .mount("/public", StaticFiles::from("/static"))
    .launch();
}
