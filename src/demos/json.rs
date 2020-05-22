// JSON
//
// Handle JSON data by using the `serde` crate and the Json type from
// rocket_contrib. The only condition is that the generic type in Json
// implements the Deserialize trait from Serde. 
//
// Add `serde` to `Cargo.toml` and include the feature "derive":
//
//     serde = { version = "1.0", features = ["derive"] }

use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemWithDeserialize {
    name: String,
    done: bool,
}

#[post("/create-item-with-deserialize-with-json", data = "<item>")]
pub fn create_item_with_deserialize_with_json(item: Json<ItemWithDeserialize>) -> String {
    format!(
        "Create item3 with json... name:{} done:{}",
        item.name,
        item.done, 
    )
}
