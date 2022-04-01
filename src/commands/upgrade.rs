use version_compare::Version;
use crate::commands::install::install;
use crate::util::database::fns::{get_all_installed, get_remote_package};
use crate::util::lock::{create_lock, lock_exists, remove_lock};

pub fn upgrade() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    // Ensure databases are synced
    crate::commands::sync::sync();

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    println!("==> Checking for updates...");

    let installed_packages = get_all_installed();
    let mut updates: Vec<String> = Vec::new();

    for i in installed_packages {
        let source = i.clone().source;
        let source_name = source.split(",").collect::<Vec<&str>>()[0];

        if source_name == "local" {
            continue;
        }

        let remote_package = get_remote_package(&i.name, &source_name.to_string());

        if remote_package.is_err() {
            continue;
        }

        let remote_package = remote_package.unwrap();

        // Always force upgrade if the epoch is higher
        if &remote_package.epoch > &i.epoch {
            updates.push(i.name.clone());
            continue;
        }

        if Version::from(&*remote_package.version) > Version::from(&*i.version) {
            updates.push(i.name.clone());
        }
    }

    match updates.len() {
        0 => {
            println!("==> No updates found.");

            remove_lock().expect("Failed to remove lock file.");
            std::process::exit(0);
        },
        1 => {
            println!("==> Updating {} package...", updates.len());
        },
        _ => {
            println!("==> Updating {} packages...", updates.len());
        }
    }

    // Append padding to updates so install will accept it
    let mut padding: Vec<String> = vec!["0".to_string(), "1".to_string()];
    padding.append(&mut updates);

    // Remove the lock as the install will takeover
    remove_lock().expect("Failed to remove lock?");

    install(padding);

    // remove_lock is done by install
}