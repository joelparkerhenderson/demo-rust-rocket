// Databases
//
// Rocket includes built-in, ORM-agnostic support for databases. In particular,
// Rocket provides a procedural macro that allows you to easily connect your
// Rocket application to databases through connection pools. A database
// connection pool is a data structure that maintains active database
// connections for later use in the application. This implementation of
// connection pooling support is based on r2d2 and exposes connections through
// request guards. Databases are individually configured through Rocket's
// regular configuration mechanisms: a Rocket.toml file, environment variables,
// or procedurally.

use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("demo_rust_rocket")]
pub struct Db(diesel::PgConnection);

#[get("/db/items/<_id>")]
pub fn get_item(conn: Db, _id: i32) -> String {
    use crate::models::*;
    use crate::diesel::prelude::*;
    use crate::schema::items::dsl::*;
    let result = items.filter(id.eq(_id)).first::<Item>(&*conn);
    match result {
        Ok(item) => format!("Get item {} demo_text:{:#?}", _id, item.demo_text),
        Err(err) => match err {
            ::diesel::result::Error::NotFound => "Not Found".into(),  // an error message akin to ::rocket::http::Status::NotFound
            _ => "Internal Server Error".into(),  // an error message akin to ::rocket::http::Status::InternalServerError
        }
    }
}


#[cfg(test)]
mod tests {

    use super::super::rocket;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn test_db() {
        let client = Client::new(rocket()).expect("rocket");
        let mut response = client.get("/db/items/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("TODO".into()));
    }

}