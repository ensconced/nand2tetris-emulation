use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

pub struct SourceModule {
    pub filename: OsString,
    pub source: String,
}

impl SourceModule {
    pub fn new(path: PathBuf) -> Self {
        let source = fs::read_to_string(&path).expect("failed to read file to string");
        Self {
            source,
            filename: path.file_name().expect("file name should not terminate in \"..\"").to_owned(),
        }
    }
}

pub fn mock_from_sources(sources: Vec<&str>) -> Vec<SourceModule> {
    sources
        .into_iter()
        .enumerate()
        .map(|(idx, source)| SourceModule {
            filename: format!("mock_file_{}", idx).into(),
            source: source.to_owned(),
        })
        .collect()
}

pub fn get_source_modules(src_path: &Path) -> Result<Vec<SourceModule>, io::Error> {
    let metadata = fs::metadata(src_path)?;
    let source_modules = if metadata.is_dir() {
        fs::read_dir(src_path)?.flatten().map(|entry| SourceModule::new(entry.path())).collect()
    } else {
        vec![SourceModule::new(src_path.to_owned())]
    };
    Ok(source_modules)
}
