// Demo of the simplest route and handler.
// When a browser does a HTTP GE request to "/"
// then this handler prints "hello world".

#[get("/hello")]
pub fn hello() -> &'static str {
    "hello world"
}
