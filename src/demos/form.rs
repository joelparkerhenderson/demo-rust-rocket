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
