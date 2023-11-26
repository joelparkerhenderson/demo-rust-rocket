# Templates

See source:
[src/demos/templates.rs](../../src/demos/templates.rs)

Rocket includes built-in templating support that works largely through a
Template responder. To render a template named "index", for instance, you
might return a value of type Template as follows:

```rust
use rocket_dyn_templates::Template;

#[get("/foo")]
fn foo() -> Template {
    let context = HashMap::new(); /* object-like value */;
    Template::render("foo", &context)
}
```

Templates are rendered with the render method. The method takes in the name of a
template and a context to render the template with. The context can be any type
that implements Serialize and serializes into an Object value, such as structs,
HashMaps, and others.

For a template to be renderable, it must first be registered. The Template
fairing automatically registers all discoverable templates when attached. The
Fairings sections of the guide provides more information on fairings. To attach
the template fairing, simply call .attach(Template::fairing()) on an instance of
Rocket as follows:

```rust
fn main() {
    rocket::ignite()
        .mount("/", routes![/* .. */])
        .attach(Template::fairing());
}
```

Rocket discovers templates in the configurable template_dir directory.
Templating support in Rocket is engine agnostic. The engine used to render a
template depends on the template file's extension. For example, if a file ends
with .hbs, Handlebars is used, while if a file ends with .tera, Tera is used.

Note: The name of the template does not include its extension.

For a template file named index.html.tera, call render("index") and use the name
"index" in templates, i.e, {% extends "index" %} or {% extends "base" %} for
base.html.tera.


## Create directory

Create any directory for the templates, such as:

```sh
mkdir -p www/templates
```


## Templating with Tera
 
Create a template file such as `www/templates/base.html.tera`:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Demo Tera</title>
  </head>
  <body>
    {% block content %}{% endblock content %}
  </body>
</html>
```

Create a content file such as `www/templates/hello.html.tera`:

```tera
{% extends "base" %}
{% block content %}
    hello world
{% endblock content %}
```
