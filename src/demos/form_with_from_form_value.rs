// The FromFormValue trait can be derived for enums with nullary fields.
//
// The derive generates an implementation of the FromFormValue trait for the
// decorated enum. The implementation returns successfully when the form value
// matches, case insensitively, the stringified version of a variant's name,
// returning an instance of said variant.

// #[macro_use] extern crate rocket;

use rocket::request::FromFormValue;

#[derive(FromFormValue)]
enum MyValue {
    First,
    Second,
    Third,
}
