use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{self, Row};
use crate::DemoRustRocketSqliteConnection;

#[get("/demo-rust-rocket-sqlite-items/<id>")]
pub async fn get_item(mut db: Connection<DemoRustRocketSqliteConnection>, id: i64) -> Option<String> {
   sqlx::query("SELECT name FROM items WHERE id = ?").bind(id)
       .fetch_one(&mut **db).await
       .and_then(|r| Ok(r.try_get(0)?))
       .ok()
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn get_item() {
        // This test needs an existing database record with:
        //
        // * id: 1
        // * name: "Call Alice"
        //
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/demo-rust-rocket-sqlite-items/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Call Alice".into()));
    }

}