// Method route 
//
// This demo that shows how to use Rocket HTTP macros for HTTP routes:
//
//   * get /users = typically returns a list of users
//   * get /users/id = typically returns a given user by id
//   * post /users/id = typically creates/updates a given user by id
//   * put /users/id = typically creates a given user by id
//   * patch /users/id = typically updates a given user by id
//   * delete /users/id = typically deletes a given user by id
//   * head /users
//   * options /users
//
// This method route demo is solely to show the macros;
// this method route demo doesn't connect to a database,
// or do any data processing of users or structures.

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

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn get_users() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Get".into()));
    }

    #[test]
    fn get_user() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Get id:1".into()));
    }

    #[test]
    fn post_user() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.post("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Post id:1".into()));
    }

    #[test]
    fn put_user() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.put("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Put id:1".into()));
    }

    #[test]
    fn patch_user() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.patch("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Patch id:1".into()));
    }

    #[test]
    fn delete_user() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.delete("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Delete id:1".into()));
    }

    #[test]
    fn head_users() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.head("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("".into()));
    }

    #[test]
    fn options_users() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.options("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Options".into()));
    }

}
