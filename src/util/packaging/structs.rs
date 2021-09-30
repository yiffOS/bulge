use serde::Deserialize;


#[derive(Deserialize)]
pub struct LocalPackage {
    name: String,
    version: String,
    epoch: i32,
    description: String,
    groups: String,
    url: String,
    license: String,
    depends: String,
    optional_depends: String,
    provides: String,
    conflicts: String,
    replaces: String,
    sha512sum: String
}