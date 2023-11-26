#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn files() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/static-files-with-a-file-server/demo1.txt").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Demo 1 of static files with a file server\n".into()));
        let response = client.get("/static-files-with-a-file-server/demo2.txt").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Demo 2 of static files with a file server\n".into()));
    }

}
