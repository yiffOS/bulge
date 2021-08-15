use kdl::{KdlValue};
use crate::get_xdg_direct;

pub enum ConfigEntry {
    IgnoredPackages,
    IgnoredGroups,
    NoUpgrade,
    Arch,
    Repos
}

/// Returns a vec of the specified config entry
pub fn get_config_entry(entry: ConfigEntry) -> Result<Vec<String>,Err> {
    // Load config file
    let config_file = kdl::parse_document(
        get_xdg_direct().find_config_file("config.kdl")?
    )?;

    match entry {
        ConfigEntry::IgnoredPackages => {}
        ConfigEntry::IgnoredGroups => {}
        ConfigEntry::NoUpgrade => {}
        ConfigEntry::Arch => {}
        ConfigEntry::Repos => {}
    }

    Ok(vec![])
}