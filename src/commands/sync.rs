use std::fs::File;
use std::io::{copy, Cursor};
use futures::SinkExt;

use isahc::http::StatusCode;
use sha2::Digest;

use crate::util::config::fns::get_sources;
use crate::util::database::fns::update_cached_repos;
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::{get, get_root};
use crate::util::mirrors::load_mirrors;

use isahc::prelude::*;
use isahc::{Body, Request, Response};
use isahc::config::RedirectPolicy;

pub fn sync() {
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

            let db_response = get(&url);

            if db_response.is_err() {
                println!("Failed to get {}. Error: {}", &url, db_response.err().unwrap());
                continue;
            }

            let mut db_response_unwrap: Response<Body> = db_response.expect("Response errored while bypassing the check");

            if db_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &url, db_response_unwrap.status());
                continue;
            }

            let mut dest = {
                File::create(format!("{}/etc/bulge/databases/cache/{}.db", get_root(), i.name)).expect("Failed to save downloaded database!")
            };

            let mut content = Cursor::new(db_response_unwrap.bytes().expect("Failed to read database bytes"));

            println!("Downloaded database for {}!", i.name);

            println!("Downloading hash for {}", i.name);

            let hash_url: String;

            if i.url.is_some() {
                hash_url = format!("{}/database.hash", i.url.clone().expect("Failed to extract custom repo url"));
            } else {
                hash_url = format!("{}/database.hash", x.replace("$repo", &*i.name));
            }

            let hash_response = get(&hash_url);

            if hash_response.is_err() {
                println!("Failed to get {}. Error: {}", &hash_url, hash_response.err().unwrap());
                continue;
            }

            let mut hash_response_unwrap: Response<Body> = hash_response.expect("Response errored while bypassing the check");

            if hash_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &hash_url, hash_response_unwrap.status());
                continue;
            }

            let hash = hash_response_unwrap.bytes().expect("Failed to read hash bytes");

            let mut sha512 = sha2::Sha512::new();

            sha512.update(content.get_ref());
            let hash_result = sha512.finalize();

            if &hash_result[..] != hash {
                println!("Database for {} failed to match with provided hash. Trying next mirror.", hash_url);
                continue;
            }

            println!("Downloaded hash for {}!", i.name);

            copy(&mut content, &mut dest).expect("Failed to copy downloaded content");

            update_cached_repos(&i.name, &String::from_utf8(hash).expect("Failed to convert hash to string"));

            break;
        }
    }

    remove_lock().expect("Failed to remove lock?");
}
