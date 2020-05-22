// Rocket's FromForm parsing is strict by default. In other words, A Form<T>
// will parse successfully from an incoming form only if the form contains the
// exact set of fields in T. Said another way, a Form<T> will error on missing
// and/or extra fields. For instance, if an incoming form contains the fields
// "a", "b", and "c" while T only contains "a" and "c", the form will not parse
// as Form<T>.
//
// Rocket allows you to opt-out of this behavior via the LenientForm data type.
// A LenientForm<T> will parse successfully from an incoming form as long as the
// form contains a superset of the fields in T. Said another way, a
// LenientForm<T> automatically discards extra fields without error. For
// instance, if an incoming form contains the fields "a", "b", and "c" while T
// only contains "a" and "c", the form will parse as LenientForm<T>.

use rocket::request::LenientForm;

#[derive(FromForm)]
pub struct Item {
    name: String,
    done: bool,
}

#[post("/create-item-with-lenient-form", data = "<item>")]
pub fn create_item_with_lenient_form(item: LenientForm<Item>) -> String {
    format!(
        "Create item with lenient form... name:{} done:{}",
        item.name,
        item.done, 
    )
}
