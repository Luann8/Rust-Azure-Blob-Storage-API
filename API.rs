#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Status;
use rocket::Data;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::Route;
use rocket_contrib::json::Json;
use std::error::Error;
use std::io::Read;
use azure_sdk_storage_blob::prelude::*;
use tokio::stream::StreamExt;

#[macro_use]
extern crate rocket;

fn routes() -> Vec<Route> {
    routes![upload_file, download_file]
}

#[post("/upload", data = "<data>")]
async fn upload_file(data: Data) -> Result<status::Custom<String>, Custom<String>> {
    let account = "your_storage_account";
    let master_key = "your_storage_account_master_key";
    let container = "your_container_name";
    let blob_name = "uploaded_file.txt";

    let mut buffer = Vec::new();
    if let Err(e) = data.open().take(1024 * 1024 * 10).read_to_end(&mut buffer) {
        return Err(Custom(Status::InternalServerError, e.to_string()));
    }

    let client = StorageAccountClient::new_access_key(account, master_key)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let container_client = client.as_container_client(container);

    let mut blob_client = container_client.as_blob_client(blob_name);
    blob_client
        .put_block_blob(buffer.as_slice())
        .execute()
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(status::Custom(
        Status::Created,
        format!("File '{}' uploaded successfully!", blob_name),
    ))
}

#[get("/download")]
async fn download_file() -> Result<status::Custom<Vec<u8>>, Custom<String>> {
    let account = "your_storage_account";
    let master_key = "your_storage_account_master_key";
    let container = "your_container_name";
    let blob_name = "uploaded_file.txt";

    let client = StorageAccountClient::new_access_key(account, master_key)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let container_client = client.as_container_client(container);

    let mut blob_client = container_client.as_blob_client(blob_name);
    let response = blob_client
        .get()
        .execute()
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut bytes = Vec::new();
    let mut stream = response.into_body();
    while let Some(item) = stream.next().await {
        bytes.extend_from_slice(&item.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?);
    }

    Ok(status::Custom(Status::Ok, bytes))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build().mount("/", routes()).launch().await?;
    Ok(())
}
