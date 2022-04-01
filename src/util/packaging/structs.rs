use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub epoch: i32,
    pub description: String,
    pub groups: String,
    pub url: String,
    pub license: String,
    pub depends: String,
    pub optional_depends: String,
    pub provides: String,
    pub conflicts: String,
    pub replaces: String,
    pub sha512sum: String
}

pub struct NewPackage {
    pub name: String,
    pub groups: String,
    pub version: String,
    pub epoch: i32,
    pub installed_files: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub dependencies: Vec<String>
}

pub struct RequestPackage {
    pub name: String,
    pub version: String,
    pub epoch: i32
}