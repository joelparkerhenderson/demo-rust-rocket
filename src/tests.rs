use super::{rocket};
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn hello_world() {
    // Create a client for our Rocket instance. It's fine to use methods
    // like expect and unwrap during testing: we want our tests to panic
    // when something goes wrong.
    let client = Client::new(rocket()).expect("valid rocket instance");
    // Create a new GET request to the "/" path, and dispatch it.
    // We get back the application's response.
    let mut response = client.get("/").dispatch();

    // Asserts
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some("hello world".into()));
}
