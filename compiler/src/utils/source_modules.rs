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
    pub fn new(filename: PathBuf) -> Self {
        let source = fs::read_to_string(&filename).expect("failed to read file to string");
        Self { source, filename }
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

pub fn get_source_modules(src_path: &Path) -> Result<HashMap<PathBuf, SourceModule>, io::Error> {
    let metadata = fs::metadata(src_path)?;
    let source_modules = if metadata.is_dir() {
        fs::read_dir(src_path)?
            .flatten()
            .map(|entry| {
                let filename: PathBuf = entry.path().file_name().unwrap().into();
                (filename, SourceModule::new(entry.path()))
            })
            .collect()
    } else {
        let filename: PathBuf = src_path.file_name().unwrap().into();
        HashMap::from([(filename, SourceModule::new(src_path.to_owned()))])
    };
    Ok(source_modules)
}
