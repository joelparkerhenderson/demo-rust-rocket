# Cookies and CookieJar

See source: 
<src/demos/cookies.rs>

See Rocket API:
<https://api.rocket.rs/v0.5/rocket/http/struct.CookieJar.html>

A reference to a CookieJar is an important, built-in request guard: it allows
you to get, set, and remove cookies. Because &CookieJar is a request guard,
an argument of its type can simply be added to a handler.

Relevant code:

```rust
use rocket::http::{Cookie, CookieJar};

#[get("/get-cookie")]
pub fn get_cookie(jar: &CookieJar<'_>) -> Option<String> {
    jar.get("alfa").map(|cookie| 
        format!("Get cookie... name:{} value:{}", cookie.name(), cookie.value()))
}

#[get("/set-cookie")]
pub fn set_cookie(mut jar: &CookieJar<'_>) -> () {
    let cookie = Cookie::new("alfa", "bravo");
    jar.add(cookie)
}
```
