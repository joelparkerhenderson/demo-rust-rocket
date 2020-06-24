#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// Rocket contrib and macros
//#[macro_use] extern crate rocket_contrib; 

// Diesel database: see demos/database.rs.
// The openssl crate comes before the diesel crate
// because this is what most linkers need to happen.
extern crate openssl;
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
        demos::method_route::get_users,
        demos::method_route::get_user,
        demos::method_route::post_user,
        demos::method_route::put_user,
        demos::method_route::patch_user,
        demos::method_route::delete_user,
        demos::method_route::head_users,
        demos::method_route::options_users,
        demos::pages::pages,
        demos::templates::hello_with_template,
        demos::cookies::get_cookie, 
        demos::cookies::set_cookie, 
        demos::form::create_item_with_form, 
        demos::form_with_lenient_form::create_item_with_lenient_form,
        demos::form_with_validation::create_item_with_star_count_with_form,
        demos::json::create_item_with_deserialize_with_json,
        demos::database::get_database_item,
        demos::upload::upload,
        

    ])
    .mount("/files", StaticFiles::from(env!("FILES_DIR")))
}

fn main() {
    rocketeer().launch();
}
