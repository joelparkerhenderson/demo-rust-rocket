// Forms
//
// Forms are one of the most common types of data handled in web applications.
// Suppose your form submission is intended to create a new todo tast "Item";
// your form has two fields: a "name" text field and "done" checkbox.
//
// The Form type implements the FromData trait as long as its generic parameter
// implements the FromForm trait. In the example below, we derived the FromForm
// trait automatically for the Item structure. FromForm can be derived for any
// structure whose fields implement FromFormValue.
//
// If a POST /todo request arrives, then the form data will automatically be
// parsed into the Item structure.
//
// If the data that arrives isn't of the correct Content-Type, then the request
// is forwarded. If the data don't parse or are invalid, then a customizable 400
// Bad Request or 422 Unprocessable Entity error is returned. 
//
// Any forward or failure can be caught by using the Option and Result types.

use rocket::request::Form;

#[derive(FromForm)]
pub struct Item {
    name: String,
    done: bool,
}

#[post("/create-item-with-form", data = "<item>")]
pub fn create_item_with_form(item: Form<Item>) -> String {
    format!(
        "Create item with form... name:{} done:{}",
        item.name,
        item.done, 
    )
}

#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn create_item_with_form_with_complete_data() {
        let complete_data = format!("name={}&done={}", "alice", "true");
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.post("/create-item-with-form")
        .header(ContentType::Form)
        .body(complete_data)
        .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Create item with form... name:alice done:true".into()));
    }

    #[test]
    fn create_item_with_form_with_incomplete_data() {
        let incomplete_data = format!("done={}", "true");
        let client = Client::new(rocketeer()).expect("rocketeer");
        let response = client.post("/create-item-with-form")
        .header(ContentType::Form)
        .body(incomplete_data)
        .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

}