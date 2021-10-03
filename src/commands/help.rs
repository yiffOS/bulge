pub fn help() {
    println!("bulge - the yiffOS package manager - v{}", crate::get_version());
    println!("usage: bulge <command> [...]");
    println!("commands:");
    println!("  bulge {{-h --help}}");
    println!("  bulge {{s sync}}");
    println!("  bulge {{u upgrade}}");
    println!("  bulge {{i install}} <package(s)>");
    println!("  bulge {{li localinstall}} <path>");
    println!("  bulge {{r remove}} <package(s)>");
    println!("  bulge info <package>");
    println!("  bulge search <package>");
    println!("  bulge list");
}