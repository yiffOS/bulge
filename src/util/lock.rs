use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_lock() -> std::io::Result<()> {
    File::create("/tmp/bulge.funny")?;
    Ok(())
}

pub fn remove_lock() -> std::io::Result<()>{
    fs::remove_file("/tmp/bulge.funny")?;
    Ok(())
}

pub fn check_lock() -> bool {
    Path::new("/tmp/bulge.funny").exists()
}