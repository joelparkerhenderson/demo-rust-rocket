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
// Edit the main function and mount the route, such as:
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
// Thus:
//
//     .mount("/files", StaticFiles::from(
//         Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("www").join("files")
//     ))
//
// See https://api.rocket.rs/v0.4/rocket_contrib/serve/struct.StaticFiles.html

// use rocket_contrib::serve::StaticFiles;

//     .mount("/files", StaticFiles::from(
//         Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("www").join("files")
//     ))
