use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/hello-with-template")]
pub fn hello_with_template() -> Template {
    let context: HashMap<String, String> = HashMap::new(); /* object-like value */
    Template::render("hello", &context)
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn hello_with_template() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/hello-with-template").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert_eq!(response.into_string(), Some("<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Demo Tera</title>\n  </head>\n  <body>\n    \n    hello world\n\n  </body>\n</html>\n".into()));
    }

}