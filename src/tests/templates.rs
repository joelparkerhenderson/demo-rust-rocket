use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn hello_with_template() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/hello-with-template").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(response.body_string(), Some("<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Demo Tera</title>\n  </head>\n  <body>\n    \n    hello world\n\n  </body>\n</html>\n".into()));
}
