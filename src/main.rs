// Import the `rocket` and `rocket_contrib` crates
#[macro_use] extern crate rocket;
use rocket::fs::TempFile;
use uuid::Uuid;
use std::path::Path;

// Define the `upload` function with the `post` attribute
#[post("/upoad", format = "plain", data = "<file>")]
async fn upload(mut file: TempFile<'_>) -> std::io::Result<()> {
    let id = Uuid::new_v4();
    let form = format!("./files/{}", id.to_string());
    let path = Path::new(&form);
    file.persist_to(path).await
}

// Define the `download` function with the `get` attribute
#[get("/download/<identifier>")]
fn download(identifier: &str) -> &'static str {
    "You did a heckin download fam!"
}

// Define the `delete` function with the `delete` attribute
#[delete("/delete/<identifier>")]
fn delete(identifier: &str) -> &'static str {
    "Well Done I wish you know you delete sth permanently from the world T_T!"
}

// Define the `replace` function with the `put` attribute
#[put("/replace/<identifier>")]
fn replace(identifier: &str) -> &'static str {
    "Woaaa that's new...!"
}

// Define the `list` function with the `get` attribute
#[get("/list")]
fn list() -> &'static str {
    "hey, here's all thefiles, they're cool!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        upload,
        download,
        delete,
        replace,
        list
    ])
}