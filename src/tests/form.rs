use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn create_item_with_form_with_complete_data() {
    let complete_data = format!("name={}&done={}", "alice", "true");
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item-with-form")
    .header(ContentType::Form)
    .body(complete_data)
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item with form... name:alice done:true".into()));
}

#[test]
fn create_item_with_form_with_incomplete_data() {
    let incomplete_data = format!("done={}", "true");
    let client = Client::new(rocket()).expect("rocket");
    let response = client.post("/create-item-with-form")
    .header(ContentType::Form)
    .body(incomplete_data)
    .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);
}
