# Demo Rust Rocket

This is a demonstration of:

* The Rust programming language

* The Rocket 0.5 web framework

This demo is a quick start of the most-important areas for the projects we build:
  
* Setup with Rocket.toml, .env.example, and .gitignore

* Static file responses per file, and with path segments, and with a file server.

* HTTP REST method routes for GET, PUT, POST, PATCH, HEAD, OPTIONS.

* JSON request and response.

* SQLite databases with sqlx and the new Rocket database connection pool.

* Templates with Tera.

* Cookies and the new Rocket Cookie Jar.
  
* Testing with system tests.

This demo focuses on simple code to get a sense of Rocket. This demo is not intended to be a production-ready application, and is not intended to cover all Rocket capabilities. For that, see the Rocket documentation.

Feedback welcome.


## Create app

Follow this guide:

* <https://rocket.rs/v0.5/guide/getting-started/>

Create an app:

```sh
cargo new demo-rust-rocket --bin
cd demo-rust-rocket
```

Edit file `Cargo.toml` to add dependency:

```toml
[dependencies]
rocket = "0.5.0"
```

Edit file `main.rs` to be:

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

### Test

Run:

```sh
cargo test
```

You should see minimal output because so far there are no tests:

```text
    Finished test [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/main.rs (target/debug/deps/demo_rust_rocket-54a4eaafb5adb876)

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


### Browse

Run:

```sh
cargo run
```

You should see:

```sh
     Running `target/debug/demo-rust-rocket`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 10
   >> max blocking threads: 512
   >> ident: Rocket
   >> IP header: X-Real-IP
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/77/xrhg507j46x_bwpzhtbmn3pw0000gn/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (index) GET /
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
🚀 Rocket has launched from http://127.0.0.1:8000
```

Browse:

* <http://localhost:8000>

You should see:

```text
Hello, world!
```

## Refactor to add a Rocket build phase

To launch a Rocket application, the suggested approach is to return an instance of `Rocket<Build>` from a function named rocket marked with the `#[launch]` attribute. 

This generates a main function with an async runtime that runs the returned Rocket instance.

Example:

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
}
```

We refactor because we prefer a more-flexible more-explicit approach. 

We separate the Rocket "build phase" from the later "ignite phase" and "launch phase". This will enable us to test more easily because we can create Rocket instances for testing.

```rust
use rocket::{Build, Rocket};

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}
```


## Refactor to add a demos module

Create a demos module directory:

```sh
mkdir src/demos
```

Create a demos module file `src/demos/mod.rs`:

```rust
pub mod index;
```

Create a file `src/demos/index.rs`:

```rust
// Demo of an index route handler, a unit test, and client test.
// When a browser client does a typical HTTP GET request to "/"
// then this handler responds with the text "Hello, world!".

#[get("/")]
pub fn handler() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {

    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn unit() {
        assert_eq!(super::handler(), "Hello, world!");
    }

    #[test]
    fn client() {
        let client = Client::tracked(rocket()).expect("rocket");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

}
```


### Test

Run:

```sh
cargo test
```

You should see the tests passed:

```text
running 2 tests
test demos::index::tests::unit ... ok
test demos::index::tests::system ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


### Browse

Run:

```sh
cargo run
```

Browse:

* <http://127.0.0.1:8000/>

You should see:

```text
Hello, world!
```



## Rocket features and dependencies

Rocket features include:

* `http2`:  Support for HTTP/2 (enabled by default).
* `json`: Support for JSON (de)serialization.
* `msgpack`: Support for MessagePack (de)serialization.
* `uuid`: Support for UUID value parsing and (de)serialization.
* `secrets`: Support for authenticated, encrypted private cookies.
* `tls`: Support for TLS encrypted connections.
* `mtls`: Support for verified clients via mutual TLS.

Options:

* For templating support, depend on `rocket_dyn_templates`. Features include templating with Handlebars and/or Tera .

* For database pools, depend on `rocket_sync_db_pools` and/or `rocket_db_pools`.

* For JSON support, depend on `serde`.
  
Edit file `Cargo.toml` to add your preferred dependencies, such as:

```toml
[dependencies]
rocket = { version = "0.5.0", features = ["http2", "json", "msgpack", "uuid", "secrets", "tls", "mtls"] }
rocket_dyn_templates = { version = "0.1.0", features = ["handlebars", "tera"] }
rocket_db_pools = { version = "0.1.0", features = ["sqlx_sqlite"] }
serde = { version = "1.0", features = ["derive"] }
```


## Create more demos


## Documentation

Setup:

* [Rocket.toml configuration file](doc/rocket-toml-configuration-file/)
* [Environment variable file .env.example](.env.example)
* [Git ignore file .gitignore](.gitignore)
* [Install sccache (Shared Compilation Cache)](doc/install-sccache/)
* [Install diesel_cli (Diesel Command Line Interface](doc/install-diesel-cli/)

Demos as source code:

* <src/demos/static_route.rs>: Static route "/hello"
* <src/demos/dynamic_route.rs>: Dynamic route "/echo"
* <src/demos/method_routes.rs>: Method routes "/users" with GET, PUT, POST, PATCH, HEAD, etc.
* <src/demos/form.rs>: Form demo
* <src/demos/upload.rs>: Upload demo

Capabilities:

* [Cookies and CookieJar](doc/cookies-and-cookie-jar/)
* [Databases](doc/databases/)
* [JSON](doc/json/)
* [Static file](doc/static-file/)
* [Static files with path segments](doc/static-files-with-path-segments/)
* [Static files with a file server](doc/static-files-with-a-file-server/)
* [Templates](doc/templates/)
  
