use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn create_item_with_form_with_validation_with_valid_data() {
    const VALID_DATA: i8 = 1;
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item-with-star-count-with-form")
    .header(ContentType::Form)
    .body(format!("name={}&star_count={}", "alice", VALID_DATA))
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item-with-star-count with form... name:alice star_count:1".into()));
}

#[test]
fn create_item_with_form_with_validation_with_invalid_data() {
    const INVALID_DATA: i8 = 0;
    let client = Client::new(rocket()).expect("rocket");
    let response = client.post("/create-item-with-star-count-with-form")
    .header(ContentType::Form)
    .body(format!("name={}&star_count={}", "alice", INVALID_DATA))
    .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);
}
