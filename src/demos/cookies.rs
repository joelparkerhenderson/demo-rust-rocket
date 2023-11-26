use rocket::http::{Cookie, CookieJar};

#[get("/get-cookie")]
pub fn get_cookie(jar: &CookieJar<'_>) -> Option<String> {
    jar.get("alfa").map(|cookie| 
        format!("Get cookie... name:{} value:{}", cookie.name(), cookie.value()))
}

#[put("/set-cookie")]
pub fn set_cookie(jar: &CookieJar<'_>) -> () {
    let cookie = Cookie::new("alfa", "bravo");
    jar.add(cookie)
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn set_cookie() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.put("/set-cookie").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn set_cookie_then_get_cookie() {
        let client = Client::tracked(rocket()).expect("rocket");
        let _ = client.put("/set-cookie").dispatch();
        let response = client.get("/get-cookie").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Get cookie... name:alfa value:bravo".into()));
    }


}