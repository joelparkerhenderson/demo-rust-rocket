// Demo of an index route handler, a unit test, and client test.
// When a browser client does a typical HTTP GET request to "/hello"
// then this handler responds with the text "Hello, world!".

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn unit() {
        assert_eq!(super::hello(), "Hello, world!");
    }

    #[test]
    fn client() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/hello").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

}
