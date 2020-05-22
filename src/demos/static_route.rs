// Demo of the simplest route and handler.
// When a browser does a HTTP GE request to "/"
// then this handler prints "hello world".

#[get("/hello")]
pub fn hello() -> &'static str {
    "hello world"
}

#[cfg(test)]

use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn test_hello() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/hello").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("hello world".into()));
}
