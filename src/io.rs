use anyhow::Result;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(file_name: P) -> Result<String> {
    fn inner(file_name: &Path) -> Result<String> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    inner(file_name.as_ref())
}
