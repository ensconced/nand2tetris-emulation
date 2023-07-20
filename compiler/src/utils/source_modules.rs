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

pub fn get_source_modules(src_path: &Path) -> Result<HashMap<PathBuf, SourceModule>, io::Error> {
    let metadata = fs::metadata(src_path)?;
    let source_modules = if metadata.is_dir() {
        fs::read_dir(src_path)?
            .flatten()
            .filter_map(|entry| {
                let filename: PathBuf = entry.path().file_name().unwrap().into();
                filename
                    .extension()
                    .map(|ext| ext.to_owned())
                    .and_then(|ext| (ext == "jack").then(|| (filename, SourceModule::new(entry.path()))))
            })
            .collect()
    } else {
        let filename: PathBuf = src_path.file_name().unwrap().into();
        HashMap::from([(filename, SourceModule::new(src_path.to_owned()))])
    };
    Ok(source_modules)
}
