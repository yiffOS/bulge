use std::collections::HashSet;
use std::iter::FromIterator;
use crate::commands::install::install;
use crate::util::database::fns::{get_group, search_for_group};
use crate::util::lock::{create_lock, lock_exists, remove_lock};

pub fn group_install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a group to install. (Check bulge --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    let requested_groups: Vec<String> = args.clone().drain(2..).collect();
    let mut install_queue: HashSet<String> = HashSet::new();

    for i in requested_groups {
        println!("==> Looking for packages in {}", &i);

        let group_repo = search_for_group(&i);

        if group_repo.is_err() {
            eprintln!("==> Group {} not found!", &i);

            remove_lock().expect("Failed to remove lock?");
            std::process::exit(1);
        }

        let requested_group = get_group(&group_repo.unwrap(), &i);

        for x in requested_group {
            install_queue.insert(x.name);
        }
    }

    // Append padding to updates so install will accept it
    let mut padding: Vec<String> = vec!["0".to_string(), "1".to_string()];
    padding.append(&mut install_queue.into_iter().collect());

    // Remove the lock as the install will takeover
    remove_lock().expect("Failed to remove lock?");

    install(padding);

    // remove_lock is done by install
}