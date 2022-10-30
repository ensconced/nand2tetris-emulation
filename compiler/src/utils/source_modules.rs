use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

pub struct SourceModule {
    pub filename: PathBuf,
    pub source: String,
}

impl SourceModule {
    pub fn new(path: PathBuf) -> Self {
        let source = fs::read_to_string(&path).expect("failed to read file to string");
        Self {
            source,
            filename: path.file_name().expect("file name should not terminate in \"..\"").to_owned().into(),
        }
    }
}

// TODO - move this into test module
pub fn mock_from_sources(sources: Vec<(&str, &str)>) -> HashMap<PathBuf, SourceModule> {
    sources
        .into_iter()
        .map(|(filename, source)| {
            (
                filename.into(),
                SourceModule {
                    filename: filename.into(),
                    source: source.to_owned(),
                },
            )
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
