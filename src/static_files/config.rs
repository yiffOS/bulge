pub fn default_config() -> &'static str {
    r#"config {
    architecture "x86_64"
    colour true
    progressbar true
    makeflags "-j4"
}

paths {
    root ""
    database ""
    cache ""
    log ""
    gpg ""
}

repos {
    core active=true {
        ignored {
            package "eee"
            group ""
        }
    }

    extra active=true {
        ignored {
            package ""
            group ""
        }
    }

    community active=true {
        ignored {
            package ""
            group ""
        }
    }
}"#
}