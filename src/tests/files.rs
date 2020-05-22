use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn files() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/files/demo.txt").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("This is a demo file.\n".into()));
}
