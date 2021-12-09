use std::fs::File;
use std::io::Write;
use std::{fs, vec};
use isahc::http::StatusCode;
use isahc::ReadResponseExt;
use rusqlite::Error;
use crate::util::config::fns::get_sources;

use crate::util::database::fns::{get_remote_package, search_for_package};
use crate::util::database::structs::{RemotePackage, Source};
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::{get, get_root};
use crate::util::mirrors::load_mirrors;
use crate::util::packaging::fns::run_install;
use crate::util::packaging::structs::RequestPackage;

struct Packages {
    name: String,
    repo: String
}

pub fn install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a package to install. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let requested_packages: Vec<String> = args.clone().drain(2..).collect();
    let mut packages: Vec<Packages> = vec![];

    for i in &requested_packages {
        let repo = search_for_package(&i);

        if repo.is_err() {
            eprintln!("ERR> {} was not found! Aborting...", i);
    
            remove_lock().expect("Failed to remove lock?");
    
            std::process::exit(1);
        }

        packages.push(Packages {
            name: i.clone(),
            repo: repo.unwrap()
        });
    }

    for i in packages {
        let remote_package = get_remote_package(&i.name, &i.repo).expect("Failed to get remote package.");

        let mut package = RequestPackage{
            name: remote_package.name.clone(),
            version: remote_package.version.clone(),
            epoch: remote_package.epoch.clone()
        };

        println!("==> Downloading {} v{}-{}...", &package.name, &package.version, &package.epoch);

        for x in load_mirrors() {
            let url = format!("{}/{}-{}-{}.tar.xz", x.replace("$repo", &*i.repo),
                              &package.name, &package.version, &package.epoch);

            let mut downloaded_package = get(&url).expect("Failed to get package.");

            if downloaded_package.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &url, downloaded_package.status());
                continue;
            }

            File::create(format!("{}/tmp/{}-{}-{}.tar.xz", get_root(),
                                 &package.name, &package.version, &package.epoch))
                .expect("Failed to create temporary file!")
                .write_all(downloaded_package.bytes().expect("Failed to get bytes.").as_slice())
                .expect("Failed to write to temporary file!");

            let mut file = File::open(format!("{}/tmp/{}-{}-{}.tar.xz", get_root(),
                                              &package.name, &package.version, &package.epoch))
                .expect("Failed to open temporary file!");

            run_install(file, &package.name, Source { name: "core".to_string(), url: Some(url) });

            fs::remove_file(format!("{}/tmp/{}-{}-{}.tar.xz", get_root(),
                                    &package.name, &package.version, &package.epoch))
                .expect("Failed to remove temporary file!");

            break;
        }

        println!("==> Installed {} v{}-{}!", &package.name, &package.version, &package.epoch);
    }


    remove_lock().expect("Failed to remove lock?");
}