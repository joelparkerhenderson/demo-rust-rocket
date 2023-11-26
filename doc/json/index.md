# JSON

See source:
[src/demos/json.rs](../../src/demos/json.rs)

Rocket makes it easy to handle JSON data by using the `serde` crate and the
Rocket Json type. The only condition is that the generic type in Json
implements the Deserialize trait from Serde. 

Add `serde` to `Cargo.toml` and include the feature "derive":

```toml
serde = { version = "1.0", features = ["derive"] }
```

Relevant code:

```rust
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
```
