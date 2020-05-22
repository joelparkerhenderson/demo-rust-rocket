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
pub fn hello_with_template() -> Template {
    let context: HashMap<String, String> = HashMap::new(); /* object-like value */;
    Template::render("hello", &context)
}
