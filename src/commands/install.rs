use crate::util::lock::{create_lock, remove_lock, lock_exists};
use crate::util::database::{search_for_package};

pub fn install(args: Vec<String>) {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/bulge.funny already exist?)");

    if args.len() < 3 {
        println!("Please provide a package to install. (Check bulge --help for usage)");
        std::process::exit(1);
    }

    let package: String = args[2].to_lowercase();
    let repo: String = search_for_package(&package);

    if repo.is_empty() {
        println!("{} was not found!", package)
    }

    println!("{} was found in {}", package, repo);

    remove_lock().expect("Failed to remove lock?");
}