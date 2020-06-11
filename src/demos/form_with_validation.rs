// Form with validation
//
// Fields of forms can be easily validated via implementations of the
// FromFormValue trait. For example, if you'd like to verify that a field
// is a integer that means a rating of 1-5 stars, then you could define
// a new StarCount type, and use it as a field in a form structure, and
// implement FromFormValue so that it validates.

use rocket::http::RawStr;
use rocket::request::{Form,FromFormValue};

#[derive(Debug)]
pub struct StarCount(usize);

impl<'v> FromFormValue<'v> for StarCount {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<StarCount, &'v RawStr> {
        match form_value.parse::<usize>() {
            Ok(x) if (x >=1 && x <= 5) => Ok(StarCount(x)),
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
pub struct ItemWithStarCount {
    name: String,
    star_count: StarCount
}

#[post("/create-item-with-star-count-with-form", data = "<item>")]
pub fn create_item_with_star_count_with_form(item: Form<ItemWithStarCount>) -> String {
    format!(
        "Create item-with-star-count with form... name:{} star_count:{}",
        item.name,
        item.star_count.0
    )
}

#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn create_item_with_form_with_validation_with_valid_data() {
        const VALID_DATA: i8 = 1;
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.post("/create-item-with-star-count-with-form")
        .header(ContentType::Form)
        .body(format!("name={}&star_count={}", "alice", VALID_DATA))
        .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("Create item-with-star-count with form... name:alice star_count:1".into()));
    }

    #[test]
    fn create_item_with_form_with_validation_with_invalid_data() {
        const INVALID_DATA: i8 = 0;
        let client = Client::new(rocketeer()).expect("rocketeer");
        let response = client.post("/create-item-with-star-count-with-form")
        .header(ContentType::Form)
        .body(format!("name={}&star_count={}", "alice", INVALID_DATA))
        .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

}