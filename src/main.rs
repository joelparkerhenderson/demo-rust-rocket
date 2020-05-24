#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// Rocket contrib and macros
//#[macro_use] extern crate rocket_contrib; 

// Diesel database: see demos/database.rs
#[macro_use] extern crate diesel;

// Environment variable settings
extern crate dotenv;

// Stactic files: see demos/static_files.rs
use rocket_contrib::serve::StaticFiles;

// Templates: see demos/templates.rs
use rocket_contrib::templates::Template;

// Paths: see demos/pages.rs
use std::path::Path;

// Database: see demos/database.rs
pub mod schema;
pub mod models;

// Load the demos
pub mod demos;

// We separate creation of the Rocket instance from launch of the instance.
// This makes testing easier, less verbose, and less error-prone.

fn rocketeer() -> rocket::Rocket {
    rocket::ignite()
    .attach(Template::fairing())
    .attach(demos::database::Db::fairing())
    .mount("/", routes![
        demos::static_route::hello,
        demos::dynamic_route::echo,
        demos::pages::pages,
        demos::templates::hello_with_template,
        demos::cookies::get_cookie, 
        demos::cookies::set_cookie, 
        demos::form::create_item_with_form, 
        demos::form_with_lenient_form::create_item_with_lenient_form,
        demos::form_with_validation::create_item_with_star_count_with_form,
        demos::json::create_item_with_deserialize_with_json,
        demos::upload::upload,
        

    ])
    .mount("/files", StaticFiles::from(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www").join("files")
    ))
}

fn main() {
    rocketeer().launch();
}
