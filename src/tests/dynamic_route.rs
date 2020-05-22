use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn echo() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/echo/foo").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("foo".into()));
}
