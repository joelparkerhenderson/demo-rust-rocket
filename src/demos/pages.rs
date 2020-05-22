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

use rocket::response::NamedFile;
use std::path::{Path,PathBuf};

#[get("/pages/<path..>")]
pub fn pages(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www").join("pages").join(path)
    ).ok()
}

#[cfg(test)]

use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn test_pages() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/pages/demo.html").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(response.body_string(), Some("This is a demo page.\n".into()));
}
