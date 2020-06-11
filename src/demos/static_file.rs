// Demo of static file capabilty.
// This shows how to serve a typical file `favicon.ico`.
// To serve multiple static files see `static_files.rs`.

use rocket::response::NamedFile;
use std::path::Path;

#[get("/favicon.ico")]
pub fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("www/assets/img/favicon/favicon.ico")).ok()
}

#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn favicon() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/favicon.ico").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::new("image", "x-icon")));
    }

}