#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

// Demo of the simplest route and handler.
// When a browser does a HTTP GE request to "/"
// then this handler prints "hello world".
//
#[get("/")]
fn hello() -> &'static str {
    "hello world"
}

// Demo of a dynamic route. This shows how to echo some text.
//
// Rocket types raw strings separately from decoded strings.
//
// Rocket provides the type RawStr that represents an unsanitized,
// unvalidated, and undecoded raw string from an HTTP message. 
// RawStr exists to separate validated string inputs, represented by
// types such as String, &str, and Cow<str>, from unvalidated inputs, 
// represented by &RawStr. RawStr also provides helpful methods to 
// convert the unvalidated string into a validated one.
//
// Because &RawStr implements FromParam, it can be used as the type of a 
// dynamic segment, as in the example above, where the value refers to a 
// potentially undecoded string. By contrast, a String is guaranteed to 
// be decoded. Which one you should use depends on whether you want 
// direct but potentially unsafe access to the string (&RawStr), or safe
// access to the string at the cost of an allocation (String).
//
use rocket::http::RawStr;

#[get("/echo/<text>")]
fn echo(text: &RawStr) -> String {
    format!("{}", text.as_str())
}

// Rocket makes it easy to serve pages by using "multiple segments".
// You can match against multiple segments by using <param..> in a 
// route path. 
//
// The type of such parameters, known as segments guards, must implement
// FromSegments. A segments guard must be the final component of a path: 
// any text after a segments guard will result in a compile-time error.
//
// Use paths and named files:
//
//     use std::path::PathBuf;
//     use rocket::response::NamedFile;
//
// Create a route and function:
//
//     #[get("/page/<path..>")]
//     fn pages(path: PathBuf) { /* ... */ }

use std::path::{Path,PathBuf};
use rocket::response::NamedFile;

#[get("/pages/<path..>")]
fn pages(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www").join("pages").join(path)
    ).ok()
}

// Rocket makes it easy to serve static files. Use the StaticFiles custom 
// handler from rocket_contrib, which makes it as simple as one line.
//
// Edit Cargo.toml and add:
//
//     rocket_contrib = "*"
//
// Edit main.rs and use the class:
//
//     use rocket_contrib::serve::StaticFiles;
//
// Edit the main function and mount the route:
//
//     rocket.mount("/files", StaticFiles::from("/var/www/public/"))
//
// The `from` function uses an absolute path to a system directory.
//
// For this demo, we don't want to use an absolute path; we want to
// use a relative path based on this demo's cargo manifest directory.
//
// The directory path is gettable via a Rust environment variable:
//
//     Path::new(env!("CARGO_MANIFEST_DIR"))
// 
// For this demo repository, we use a directory `www/files` and put te
// static files there; note that we could have chosen any directory name.
//
// So the path is:
//
//     Path::new(env!("CARGO_MANIFEST_DIR")).join("www").join("files"))Path::new(env!("CARGO_MANIFEST_DIR"))
//
// See https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html

use rocket_contrib::serve::StaticFiles;

// Templates
//
// Rocket includes built-in templating support that works largely through a
// Template responder in rocket_contrib. To render a template named "index", for
// instance, you might return a value of type Template as follows:
//
//     use rocket_contrib::templates::Template;
//
//     #[get("/foo")]
//     fn foo() -> Template {
//         let context = HashMap::new(); /* object-like value */;
//         Template::render("foo", &context)
//     }
//
// Templates are rendered with the render method. The method takes in the name
// of a template and a context to render the template with. The context can be
// any type that implements Serialize and serializes into an Object value, such
// as structs, HashMaps, and others.
//
// For a template to be renderable, it must first be registered. The Template
// fairing automatically registers all discoverable templates when attached. The
// Fairings sections of the guide provides more information on fairings. To
// attach the template fairing, simply call .attach(Template::fairing()) on an
// instance of Rocket as follows:
//
//     fn main() {
//         rocket::ignite()
//             .mount("/", routes![/* .. */])
//             .attach(Template::fairing());
//     }
//
// Rocket discovers templates in the configurable template_dir directory.
// Templating support in Rocket is engine agnostic. The engine used to render a
// template depends on the template file's extension. For example, if a file
// ends with .hbs, Handlebars is used, while if a file ends with .tera, Tera is
// used.
//
// Note: The name of the template does not include its extension.
//
// For a template file named index.html.tera, call render("index") and use the
// name "index" in templates, i.e, {% extends "index" %} or {% extends "base" %}
// for base.html.tera.

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/hello-with-template")]
fn hello_with_template() -> Template {
    let context: HashMap<String, String> = HashMap::new(); /* object-like value */;
    Template::render("hello", &context)
}

// Cookies
//
// Cookies ia built-in request guard: it allows you to get, set, and 
// remove cookies. Because Cookies is a request guard, an argument of 
// its type can simply be added to a handler.
//
// The code below results in the incoming request's cookies being 
// accessible from the handler. The example above retrieves a cookie 
// named "message". Cookies can also be set and removed using the 
// Cookies guard. The cookies example on GitHub illustrates further
//  use of the Cookies type to get and set cookies, while the Cookies
// documentation contains complete usage information.

use rocket::http::{Cookie,Cookies};

#[get("/get-cookie")]
fn get_cookie(cookies: Cookies) -> Option<String> {
    cookies.get("alpha").map(|cookie| 
        format!("Get cookie... name:{} value:{}", cookie.name(), cookie.value()))
}

#[get("/set-cookie")]
fn set_cookie(mut cookies: Cookies) -> Option<String> {
    cookies.add(Cookie::new("alpha", "bravo"));
    cookies.get("alpha").map(|cookie| 
        format!("Set cookie... name:{} value:{}", cookie.name(), cookie.value())
    )
}

// Forms
//
// Forms are one of the most common types of data handled in web applications.
// Suppose your form submission is intended to create a new todo tast "Item";
// your form has two fields: a "name" text field and "done" checkbox.
//
// The Form type implements the FromData trait as long as its generic parameter
// implements the FromForm trait. In the example below, we derived the FromForm
// trait automatically for the Item structure. FromForm can be derived for any
// structure whose fields implement FromFormValue.
//
// If a POST /todo request arrives, then the form data will automatically be
// parsed into the Item structure.
//
// If the data that arrives isn't of the correct Content-Type, then the request
// is forwarded. If the data don't parse or are invalid, then a customizable 400
// Bad Request or 422 Unprocessable Entity error is returned. 
//
// Any forward or failure can be caught by using the Option and Result types.

use rocket::request::Form;

#[derive(FromForm)]
struct Item1 {
    name: String,
    done: bool,
}

#[post("/create-item1-with-form", data = "<item>")]
fn create_item1_with_form(item: Form<Item1>) -> String {
    format!(
        "Create item1 with form... name:{} done:{}",
        item.name,
        item.done, 
    )
}

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

#[post("/create-item1-with-lenient-form", data = "<item>")]
fn create_item1_with_lenient_form(item: LenientForm<Item1>) -> String {
    format!(
        "Create item1 with lenient form... name:{} done:{}",
        item.name,
        item.done, 
    )
}

// Form field validation
//
// Fields of forms can be easily validated via implementations of the
// FromFormValue trait. For example, if you'd like to verify that a field
// is a integer that means a rating of 1-5 stars, then you could define
// a new StarCount type, and use it as a field in a form structure, and
// implement FromFormValue so that it validates.

use rocket::request::FromFormValue;

#[derive(Debug)]
struct StarCount(usize);

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
struct ItemWithStarCount {
    name: String,
    star_count: StarCount
}

#[post("/create-item-with-star-count-with-form", data = "<item>")]
fn create_item_with_star_count_with_form(item: Form<ItemWithStarCount>) -> String {
    format!(
        "Create item-with-star-count with form... name:{} star_count:{}",
        item.name,
        item.star_count.0
    )
}

// The FromFormValue trait can be derived for enums with nullary fields.
//
// The derive generates an implementation of the FromFormValue trait for the
// decorated enum. The implementation returns successfully when the form value
// matches, case insensitively, the stringified version of a variant's name,
// returning an instance of said variant.

#[derive(FromFormValue)]
enum MyValue {
    First,
    Second,
    Third,
}

// JSON
//
// Handle JSON data by using the `serde` crate and the Json type from
// rocket_contrib. The only condition is that the generic type in Json
// implements the Deserialize trait from Serde. 
//
// Add `serde` to `Cargo.toml` and include the feature "derive":
//
//     serde = { version = "1.0", features = ["derive"] }

use serde::Deserialize;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
struct ItemWithDeserialize {
    name: String,
    done: bool,
}

#[post("/create-item-with-deserialize-with-json", data = "<item>")]
fn create_item_with_deserialize_with_json(item: Json<ItemWithDeserialize>) -> String {
    format!(
        "Create item3 with json... name:{} done:{}",
        item.name,
        item.done, 
    )
}

// Streaming
//
// Sometimes you want to handle incoming data directly. For example, you might
// want to stream the incoming data out to a file. Rocket does this via the Data
// type.
//
// The route below accepts any POST request to the /upload path with
// Content-Type: text/plain The incoming data is streamed out to tmp/upload.txt,
// and the number of bytes written is returned as a plain text response if the
// upload succeeds. If the upload fails, an error response is returned. The
// handler above is done. It really is that simple! See the GitHub example
// code for the full crate.
//
// Warning: You should always set limits when reading incoming data.
//
// To prevent DoS attacks, limit the amount of data you're willing to accept. 
// The take() reader adapter makes doing this easy:
//
//     data.open().take(LIMIT).
//
// TODO make the LIMIT work

use rocket::Data;

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error> {
    data.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
}

// We separate creation of the Rocket instance from launch of the instance.
// This makes testing easier, less verbose, and less error-prone.

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", routes![
        hello,
        echo,
        pages,
        hello_with_template,
        get_cookie, 
        set_cookie, 
        create_item1_with_form, 
        create_item1_with_lenient_form,
        create_item_with_star_count_with_form,
        create_item_with_deserialize_with_json,
        upload,
    ])
    .mount("/files", StaticFiles::from(
        Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("www").join("files")
    ))
}

fn main() {
    rocket().launch();
}
