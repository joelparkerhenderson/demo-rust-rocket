use super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn hello() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("hello world".into()));
}

#[test]
fn echo() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/echo/foo").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("foo".into()));
}

#[test]
fn pages() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/pages/demo.html").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert_eq!(response.body_string(), Some("This is a demo page.\n".into()));
}

#[test]
fn files() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.get("/files/demo.txt").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("This is a demo file.\n".into()));
}

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

#[test]
fn create_item1_with_form_with_complete_data() {
    let complete_data = format!("name={}&done={}", "alice", "true");
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item1-with-form")
    .header(ContentType::Form)
    .body(complete_data)
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item1 with form... name:alice done:true".into()));
}

#[test]
fn create_item1_with_form_with_incomplete_data() {
    let incomplete_data = format!("done={}", "true");
    let client = Client::new(rocket()).expect("rocket");
    let response = client.post("/create-item1-with-form")
    .header(ContentType::Form)
    .body(incomplete_data)
    .dispatch();
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[test]
fn create_item1_with_form() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item1-with-form")
    .header(ContentType::Form)
    .body(format!("name={}&done={}", "alice", "true"))
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item1 with form... name:alice done:true".into()));
}

#[test]
fn create_item1_with_lenient_form() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item1-with-lenient-form")
    .header(ContentType::Form)
    .body(format!("name={}", "alice"))
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item1 with lenient form... name:alice done:false".into()));
}

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

// TODO Add test for upload
