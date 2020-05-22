// Dnamic route
//
// This demo shows how to echo some text.
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

use rocket::http::RawStr;

#[get("/echo/<text>")]
pub fn echo(text: &RawStr) -> String {
    format!("{}", text.as_str())
}
