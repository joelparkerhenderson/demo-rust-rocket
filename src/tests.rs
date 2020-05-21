use super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn hello() {
    // Create a client for our Rocket instance. It's fine to use methods
    // like expect and unwrap during testing: we want our tests to panic
    // when something goes wrong.
    let client = Client::new(rocket()).expect("rocket");
    // Create a new GET request to the "/" path, and dispatch it.
    // We get back the application's response.
    let mut response = client.get("/").dispatch();

    // Asserts
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

// TODO add test for pages, files, cookies

#[test]
fn create_item1_with_form() {
    let client = Client::new(rocket()).expect("rocket");
    let mut response = client.post("/create-item1-with-form")
    .header(ContentType::Form)
    .body(format!("name={}&complete={}", "alice", "true"))
    .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("Create item1 with form... name:alice complete:true".into()));
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
    assert_eq!(response.body_string(), Some("Create item1 with lenient form... name:alice complete:false".into()));
}
