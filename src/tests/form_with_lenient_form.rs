use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn create_item_with_lenient_form() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item1-with-lenient-form")
    .header(ContentType::Form)
    .body(format!("name={}", "alice"))
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item1 with lenient form... name:alice done:false".into()));
}
