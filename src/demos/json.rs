use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    name: String,
    done: bool,
}

#[post("/create-item-with-json", data = "<item>")]
pub fn create_item_with_json(item: Json<Item>) -> Value {
    json!({ 
        "name": item.name, 
        "done": item.done,
    })
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn create_item_with_json() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.post("/create-item-with-json")
        .body("{\"name\":\"alice\",\"done\":true}")
        .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(response.into_string(), Some(String::from("{\"done\":true,\"name\":\"alice\"}")));
    }

}