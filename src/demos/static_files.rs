// Rocket makes it easy to serve static files. Use the StaticFiles custom 
// handler from rocket_contrib, which makes it as simple as one line.
//
// See https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html
//
// Edit Cargo.toml and add:
//
//     rocket_contrib = "*"
//
// Edit main.rs and use the class:
//
//     use rocket_contrib::serve::StaticFiles;
//
// Edit the main function and mount the route, such as:
//
//     rocket.mount("/files", StaticFiles::from("/var/www/public/"))
//
// The `from` function uses an absolute path to a system directory.
//
// We create an environment variable FILES_DIR to point to a dir:
//
//     export FILES_DIR="/foo/bar"
//
// So the mount code is:
//
//     .mount("/files", StaticFiles::from(env!("FILES_DIR"))
//
// If we instead wanted to use a relative path based on this demo's
// cargo manifest directory, the directory path is gettable via a 
// Rust environment variable that is set by cargo:
//
//     env!("CARGO_MANIFEST_DIR")
//
// So the mount code is:
//
//     .mount("/files", StaticFiles::from(
//         Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("www").join("files")
//
#[cfg(test)]
mod tests {

    use crate::rocketeer;
    use rocket::local::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn files() {
        let client = Client::new(rocketeer()).expect("rocketeer");
        let mut response = client.get("/files/demo.txt").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some("This is a demo file.\n".into()));
    }

}