#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};

// For src/demos/static_files_with_a_file_server.rs

use rocket::fs::FileServer;
use std::path::Path;

// For src/demos/templates.rs

use rocket_dyn_templates::Template;

// For src/demos/databases.rs

use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

#[derive(Database)]
#[database("demo_rust_rocket_sqlite")]
pub struct DemoRustRocketSqliteConnection(sqlx::SqlitePool);

// For Diesel database
//
// The openssl crate comes before the diesel crate
// because this is what most linkers need to happen.

//extern crate openssl;
//#[macro_use] extern crate diesel;

// For environment variable settings

//extern crate dotenv;

// Modules

pub mod demos;

// We separate creation of the Rocket instance from launch of the instance.
// This makes testing easier, clearer, and more reliable.


fn rocket() -> Rocket<Build> {
    rocket::build()
    .attach(
        Template::fairing())
    .attach(
        DemoRustRocketSqliteConnection::init()
    )
    .mount("/", routes![
        demos::index::handler,
        demos::static_route::hello,
        demos::dynamic_route::echo,
        demos::method_routes::get_users,
        demos::method_routes::get_user,
        demos::method_routes::post_user,
        demos::method_routes::put_user,
        demos::method_routes::patch_user,
        demos::method_routes::delete_user,
        demos::method_routes::head_users,
        demos::method_routes::options_users,
        demos::form::create_item_with_form,
        //demos::form_with_lenient_form::create_item_with_lenient_form,
        //demos::form_with_validation::create_item_with_star_count_with_form,
        demos::static_file::get,
        demos::static_files_with_path_segments::get,
        demos::templates::hello_with_template,
        demos::cookies::get_cookie, 
        demos::cookies::set_cookie, 
        demos::json::create_item_with_json,
        demos::databases::get_item,
        demos::upload::upload,        
    ])
    .mount(
        "/static-files-with-a-file-server", 
        FileServer::from(
            Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("www")
            .join("static-files-with-a-file-server")
        )
    )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}
