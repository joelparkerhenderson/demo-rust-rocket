use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn pages() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/pages/demo.html").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(response.body_string(), Some("This is a demo page.\n".into()));
}
