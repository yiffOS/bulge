pub fn default_config() -> &'static str {
    r#"{
        "architecture": "x86_64",
        "toolchain": "knot",
        "colour": true,
        "progressbar": true,
        "repos": [
            {
                "name": "core",
                "active": true
            },
            {
                "name": "extra",
                "active": true
            },
            {
                "name": "community",
                "active": true
            },
            {
                "name": "external",
                "active": false,
                "url": "https://www.example.com"
            }
        ]
    }"#
}

pub fn default_mirrorlist() -> &'static str {
    r#"# yiffOS default mirror list

https://repo.yiffos.gay/$repo/$arch/$toolchain"#
}