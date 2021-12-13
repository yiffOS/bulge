use std::{fs, vec};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{stdin, stdout, Write};

use isahc::http::StatusCode;
use isahc::ReadResponseExt;
use rusqlite::Error;
use text_io::read;

use crate::util::config::fns::get_sources;
use crate::util::database::fns::{get_remote_package, search_for_package};
use crate::util::database::structs::{RemotePackage, Source};
use crate::util::lock::{create_lock, lock_exists, remove_lock};
use crate::util::macros::{continue_prompt, display_installing_packages, get, get_root};
use crate::util::mirrors::load_mirrors;
use crate::util::packaging::fns::run_install;
use crate::util::packaging::structs::{Package, RequestPackage};
use crate::util::transactions::dependencies::{run_depend_check, run_depend_resolve};

#[derive(PartialEq, Eq, Hash, Clone)]
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
    let mut packages: HashSet<Packages> = HashSet::new();

    println!("==> Resolving packages and dependencies...");
    for i in &requested_packages {
        let repo = search_for_package(&i);

        if repo.is_err() {
            eprintln!("ERR> {} was not found! Aborting...", i);
    
            remove_lock().expect("Failed to remove lock?");
    
            std::process::exit(1);
        }

        let repo_unwrap = repo.unwrap();

        packages.insert(Packages {
            name: i.clone(),
            repo: repo_unwrap.clone()
        });

        let remote_package = get_remote_package(&i, &repo_unwrap).expect("Failed to get remote package.");

        if remote_package.depends.is_empty() {
            // Let's not check for depends as there is none
            continue;
        }

        let checked_deps = run_depend_check(
            run_depend_resolve(
                get_remote_package(&i, &repo_unwrap).expect("Failed to get remote package.")
            )
        );

        for x in checked_deps.iter() {
            if !x.1 {
                let repo = search_for_package(&x.0);

                if repo.is_err() {
                    eprintln!("ERR> {} was not found! Aborting...", i);

                    remove_lock().expect("Failed to remove lock?");

                    std::process::exit(1);
                }

                packages.insert(Packages {
                    name: x.0.clone(),
                    repo: repo.unwrap()
                });
            }
        }
    }

    println!("==> Checking for already installed packages...");
    // TODO: Check for already installed packages and collect them into a hashset to display
    let mut installed_packages: HashSet<Package> = HashSet::new();

    println!("==> Looking for package conflicts...");
    // TODO: Check for conflicts

    println!("==> Generating install queue...");
    let mut queue: HashMap<Package, String> = HashMap::new();
    for i in packages.clone() {
        queue.insert(
            get_remote_package(&i.name, &i.repo).expect("Failed to get remote package."),
            i.repo.clone()
        );
    }

    println!("\nPackages to install [{}]: {}\n", packages.len(), display_installing_packages(queue.clone()));

    if !(continue_prompt()) {
        println!("Abandoning install!");
        remove_lock().expect("Failed to remove lock?");
        std::process::exit(1);
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