// Dynamic route
//
// This demo shows how to echo some text.

#[get("/echo/<text>")]
pub fn echo(text: &str) -> String {
    format!("{}", text)
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn unit() {
        assert_eq!(super::echo("foo"), "foo");
    }

    #[test]
    fn client() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/echo/foo").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("foo".into()));
    }

}