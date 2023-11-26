// Static files with path segments
//
// Rocket makes it easy to serve pages by using "multiple segments".
// You can match against multiple segments by using <param..> in a 
// route path. 
//
// The type of such parameters, known as segments guards, must implement
// FromSegments. A segments guard must be the final component of a path: 
// any text after a segments guard will result in a compile-time error.
//
// Create any directory and any files, such as:
//
// ```sh
// mkdir -p www/static-files-with-path-segments
// cd $_
// echo "Demo 1 of static files with path segments" > demo1.html
// echo "Demo 2 of static files with path segments" > demo2.html
// ```

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/static-files-with-path-segments/<path..>")]
pub async fn get(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www")
        .join("static-files-with-path-segments")
        .join(path)
    ).await.ok()
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn pages() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/static-files-with-path-segments/demo1.html").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert_eq!(response.into_string(), Some("Demo 1 of static files with path segments\n".into()));
        let response = client.get("/static-files-with-path-segments/demo2.html").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert_eq!(response.into_string(), Some("Demo 2 of static files with path segments\n".into()));
    }

}