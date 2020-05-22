use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn get_cookie() {
    let client = Client::new(rocket()).expect("rocket");
    let _ = client.get("/set-cookie").dispatch();
    let mut response = client.get("/get-cookie").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Get cookie... name:alpha value:bravo".into()));
}

#[test]
fn set_cookie() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/set-cookie").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Set cookie... name:alpha value:bravo".into()));
}
