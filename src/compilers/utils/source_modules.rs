use std::{
    ffi::OsString,
    fs::{self, Metadata},
    io,
    path::{Path, PathBuf},
};

pub struct SourceModule {
    pub filename: OsString,
    pub source: String,
    pub entrypoint_is_dir: bool,
}

impl SourceModule {
    pub fn new(path: PathBuf, entrypoint_is_dir: bool) -> Self {
        let source = fs::read_to_string(&path).expect("failed to read file to string");
        Self {
            source,
            filename: path
                .file_name()
                .expect("file name should not terminate in \"..\"")
                .to_owned(),
            entrypoint_is_dir,
        }
    }
}

pub fn get_source_modules(src_path: &Path) -> Result<Vec<SourceModule>, io::Error> {
    let metadata = fs::metadata(src_path)?;
    let source_modules = if metadata.is_dir() {
        fs::read_dir(src_path)?
            .flatten()
            .map(|entry| SourceModule::new(entry.path(), true))
            .collect()
    } else {
        vec![SourceModule::new(src_path.to_owned(), false)]
    };
    Ok(source_modules)
}
