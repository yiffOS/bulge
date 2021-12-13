use std::path::Path;

pub struct ConflictingFiles {
    pub is_conflict: bool,
    pub files: Vec<String>,
}

pub fn run_conflict_check(files: &Vec<String>, is_installed: bool, root: String) -> ConflictingFiles {
    let mut conflicting_struct = ConflictingFiles {
        is_conflict: false,
        files: vec![]
    };

    for i in files {
        if !is_installed && Path::new(format!("{}{}", &root, &i).as_str()).exists() {
            conflicting_struct.is_conflict = true;
            conflicting_struct.files.push(format!("{}{}", root.clone(), i.clone()));
        }
    }

    return conflicting_struct;
}