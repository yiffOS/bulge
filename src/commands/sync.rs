use crate::util::lock::{create_lock, remove_lock, lock_exists};
use crate::util::conf::get_sources;
use crate::util::mirrors::load_mirrors;
use crate::util::database::update_cached_repos;
use reqwest::{StatusCode, Response};
use std::fs::File;
use std::io::{copy, Cursor};
use sha2::Digest;
use hex_literal::hex;
use tokio::io::AsyncReadExt;

pub async fn sync() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    println!("Synchronizing repo databases...");

    for i in get_sources() {
        println!("Downloading database for {}", i.name);

        let mirror_list = load_mirrors();

        for x in mirror_list {
            let url: String;

            if i.url.is_some() {
                url = format!("{}/database.db", i.url.clone().expect("Failed to extract custom repo url"));
            } else {
                url = format!("{}/database.db", x.replace("$repo", &*i.name));
            }

            let db_response = reqwest::get(&url).await;

            if db_response.is_err() {
                println!("Failed to get {}. Error: {}", &url, db_response.err().unwrap());
                continue;
            }

            let mut db_response_unwrap: Response = db_response.expect("Response errored while bypassing the check");

            if db_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &url, db_response_unwrap.status());
                continue;
            }

            let mut dest = {
                db_response_unwrap
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() {None} else { Some(name) })
                    .expect("Empty file name?");

                File::create(format!("/etc/bulge/databases/cache/{}.db", i.name)).expect("Failed to save downloaded database!")
            };

            let mut content = Cursor::new(db_response_unwrap.bytes().await.expect("Failed to read downloaded content"));

            println!("Downloaded database for {}!", i.name);

            println!("Downloading hash for {}", i.name);

            let hash_url: String;

            if i.url.is_some() {
                hash_url = format!("{}/database.hash", i.url.clone().expect("Failed to extract custom repo url"));
            } else {
                hash_url = format!("{}/database.hash", x.replace("$repo", &*i.name));
            }

            let hash_response = reqwest::get(&hash_url)
                .await;

            if hash_response.is_err() {
                println!("Failed to get {}. Error: {}", &hash_url, hash_response.err().unwrap());
                continue;
            }

            let hash_response_unwrap: Response = hash_response.expect("Response errored while bypassing the check");

            if hash_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &hash_url, hash_response_unwrap.status());
                continue;
            }

            let hash = hash_response_unwrap.text().await.expect("Failed to convert hash to string");

            //let mut sha512 = sha2::Sha512::new();

            //sha512.update(content.clone());
            //let hash_result = sha512.finalize();

            /* FIXME: Figure out how to convert the string hash to [u8]
            if &hash_result[..] != hex!("{}", hash) {
                println!("Database for {} failed to match with provided hash. Trying next mirror.", hash_url);
                continue;
            }
             */

            println!("Downloaded hash for {}!", i.name);

            copy(&mut content, &mut dest).expect("Failed to copy downloaded content");

            update_cached_repos(&i.name, &hash);

            break;
        }
    }

    remove_lock().expect("Failed to remove lock?");
}
