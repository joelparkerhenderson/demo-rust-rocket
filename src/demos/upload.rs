// Upload using streaming
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

use rocket::data::{Data, ToByteUnit};

#[post("/upload", format = "plain", data = "<data>")]
pub async fn upload(data: Data<'_>) -> std::io::Result<String> {
    data.open(1.megabytes()).into_file("/tmp/upload.txt").await?;
    Ok(String::from("Uploaded"))
}

// TODO
// #[cfg(test)]
// mod tests {

// use crate::rocketeer;
// use rocket::local::Client;
// use rocket::http::{ContentType, Status};

// #[test]
// fn upload() {
//     let client = Client::new(rocketeer()).expect("rocketeer");
//     //TODO
// }

//}