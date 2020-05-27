// Method route demo

#[get("/users")]
pub fn get_users() -> String {
    "Get".into()
}

#[get("/users/<_id>")]
pub fn get_user(_id: i32) -> String {
    format!("Get id:{}", _id)
}

#[post("/users/<_id>")]
pub fn post_user(_id: i32) -> String {
    format!("Post id:{}", _id)
}

#[put("/users/<_id>")]
pub fn put_user(_id: i32) -> String {
    format!("Put id:{}", _id)
}

#[patch("/users/<_id>")]
pub fn patch_user(_id: i32) -> String {
    format!("Patch id:{}", _id)
}

#[delete("/users/<_id>")]
pub fn delete_user(_id: i32) -> String {
    format!("Delete id:{}", _id)
}

#[head("/users")]
pub fn head_users() -> String {
    "Head".into()
}

#[options("/users")]
pub fn options_users() -> String {
    "Options".into()
}

#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_get_users() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Get".into()));
    }

    #[test]
    fn test_get_user() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Get id:1".into()));
    }

    #[test]
    fn test_post_user() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.post("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Post id:1".into()));
    }

    #[test]
    fn test_put_user() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.put("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Put id:1".into()));
    }

    #[test]
    fn test_patch_user() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.patch("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Patch id:1".into()));
    }

    #[test]
    fn test_delete_user() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.delete("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Delete id:1".into()));
    }

    #[test]
    fn test_head_users() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.head("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("".into()));
    }

    #[test]
    fn test_options_users() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.options("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Options".into()));
    }

}
