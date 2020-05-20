#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// Demo of the simplest route and handler.
// When a browser does a HTTP GE request to "/"
// then this handler prints "Hello world".
//
#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

// Demo of a dynamic route. This shows how to echo some text.
//
// Rocket types raw strings separately from decoded strings.
//
// Rocket provides the type RawStr that represents an unsanitized,
// unvalidated, and undecoded raw string from an HTTP message. 
// RawStr exists to separate validated string inputs, represented by
// types such as String, &str, and Cow<str>, from unvalidated inputs, 
// represented by &RawStr. RawStr also provides helpful methods to 
// convert the unvalidated string into a validated one.
//
// Because &RawStr implements FromParam, it can be used as the type of a 
// dynamic segment, as in the example above, where the value refers to a 
// potentially undecoded string. By contrast, a String is guaranteed to 
// be decoded. Which one you should use depends on whether you want 
// direct but potentially unsafe access to the string (&RawStr), or safe
// access to the string at the cost of an allocation (String).
//
use rocket::http::RawStr;

#[get("/echo/<text>")]
fn echo(text: &RawStr) -> String {
    format!("{}!", text.as_str())
}

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
// Edit the main function and mount the route:
//
//     rocket.mount("/files", StaticFiles::from("/var/www/public/"))
//
// The `from` function uses an absolute path to a system directory.
//
// For this demo, we don't want to use an absolute path; we want to
// use a relative path based on this demo's cargo manifest directory.
//
// The directory path is gettable via a Rust environment variable:
//
//     Path::new(env!("CARGO_MANIFEST_DIR"))
// 
// For this demo repository, we use a directory `www/files` and put te
// static files there; note that we could have chosen any directory name.
//
// So the path is:
//
//     Path::new(env!("CARGO_MANIFEST_DIR")).join("www").join("files"))Path::new(env!("CARGO_MANIFEST_DIR"))
//
// See https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html
//
use rocket_contrib::serve::StaticFiles;

// Cookies
//
// Cookies ia built-in request guard: it allows you to get, set, and 
// remove cookies. Because Cookies is a request guard, an argument of 
// its type can simply be added to a handler.
//
// The code below results in the incoming request's cookies being 
// accessible from the handler. The example above retrieves a cookie 
// named "message". Cookies can also be set and removed using the 
// Cookies guard. The cookies example on GitHub illustrates further
//  use of the Cookies type to get and set cookies, while the Cookies
// documentation contains complete usage information.

use rocket::http::Cookies;

#[get("/cookies")]
fn index(cookies: Cookies) -> Option<String> {
    cookies.get("message")
        .map(|value| format!("Message: {}", value))
}

// Main

fn main() {
    rocket::ignite()
    .mount("/", routes![index, pages])
    .mount("/files", StaticFiles::from(Path::new(env!("CARGO_MANIFEST_DIR")).join("www").join("files")))
    .launch();
}
