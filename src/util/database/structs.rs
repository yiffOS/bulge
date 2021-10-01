pub struct Package {
    pub name: String,
    pub version: String,
    pub epoch: i32,
    pub description: String,
    pub groups: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub build_depends: Vec<String>,
    pub optional_depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub checksum: String
}

pub struct InstalledPackages {
    pub name: String,
    pub groups: Vec<String>,
    pub source: String,
    pub version: String,
    pub epoch: i32
}

pub struct Source {
    pub name: String,
    pub url: Option<String>
}