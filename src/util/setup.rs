use std::fs;
use crate::util::lock::{lock_exists, create_lock, remove_lock};
use crate::util::database::init_database;
use std::io::Write;
use crate::static_files::config::default_config;
use crate::static_files::mirrors::default_mirrorlist;
use crate::commands::sync::sync;

pub async fn init() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    println!("Welcome to bulge!");
    println!("We'll be creating the necessary folders on root.");
    println!();

    println!("Creating /etc/bulge");
    fs::create_dir_all("/etc/bulge").expect("Failed to create /etc/bulge");

    println!("Creating /etc/bulge/databases");
    fs::create_dir_all("/etc/bulge/databases").expect("Failed to create /etc/bulge/databases");

    println!("Creating /etc/bulge/databases/cache");
    fs::create_dir_all("/etc/bulge/databases/cache").expect("Failed to create /etc/bulge/databases/cache");

    println!();
    println!("We'll now create some default files.");
    println!();

    println!("Creating default configuration file.");
    fs::File::create("/etc/bulge/config.kdl")
        .expect("Failed to create /etc/bulge/config.kdl")
        .write_all(default_config().as_ref())
        .expect("Failed to insert default configuration.");

    println!("Creating default mirror list for yiffOS.");
    fs::File::create("/etc/bulge/mirrors")
        .expect("Failed to create /etc/bulge/mirrors")
        .write_all(default_mirrorlist().as_ref())
        .expect("Failed to insert default mirror list.");

    println!("Creating default databases.");
    init_database();

    println!();
    println!("The databases will now be synced with the mirrors.");
    println!();
    remove_lock().expect("Failed to remove lock?"); // Work around for lock issues
    sync().await;

    println!();
    println!("Setup complete!");
}