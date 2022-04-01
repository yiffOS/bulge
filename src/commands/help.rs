pub fn help() {
    println!("bulge - the yiffOS package manager - v{}", crate::get_version());
    println!("usage: bulge <command> [...]");
    println!("commands:");
    println!("\t bulge {{-h --help}}");
    println!("\t\t - List all commands for bulge (this view)");
    println!("\t bulge {{s sync}}");
    println!("\t\t - Synchronizes package databases with remotes");
    println!("\t bulge {{u upgrade}}");
    println!("\t\t - Check for (and then install) package updates");
    println!("\t bulge {{i install}} <package(s)>");
    println!("\t\t - Install a specified package");
    println!("\t bulge {{li localinstall}} <path(s)>");
    println!("\t\t - Install a package from a local archive");
    println!("\t bulge {{r remove}} <package(s)>");
    println!("\t\t - Uninstall a specified package");
    println!("\t bulge info <package>");
    println!("\t\t - TODO");
    println!("\t bulge search <package>");
    println!("\t\t - TODO");
    println!("\t bulge list");
    println!("\t\t - List all installed packages with their version and source");
}