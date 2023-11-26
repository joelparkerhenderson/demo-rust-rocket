use rocket::fs::NamedFile;
use std::path::Path;

#[get("/static-file")]
pub async fn get() -> Option<NamedFile> {
    NamedFile::open(
        Path::new("www/static-file/demo.html")
    ).await.ok()
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn get() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/static-file").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert_eq!(response.into_string(), Some("Demo of a static file\n".into()));
    }

}   