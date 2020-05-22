use super::super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn create_item_with_json() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item-with-deserialize-with-json")
    .body("{\"name\":\"alice\",\"done\":true}")
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item3 with json... name:alice done:true".into()));
}
