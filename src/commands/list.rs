use crate::util::database::fns::get_all_installed;

pub fn list() {
    let result = get_all_installed();

    for i in result {
        let source = i.clone().source;
        let source_name = source.split(",").collect::<Vec<&str>>()[0];

        println!("{} {}-{} {}", i.name, i.version, i.epoch, source_name);
    }
}