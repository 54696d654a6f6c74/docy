use crate::settings::Settings;
use walkdir::{DirEntry, WalkDir};

fn is_excluded(file: &DirEntry, settings: &Settings) -> bool {
    let file_path = file
        .path()
        .to_str()
        .expect(&format!("Failed to parse file path {:?}", file.path()));

    let file_name = file
        .file_name()
        .to_str()
        .expect(&format!("Failed to parse file name {:?}", file.file_name()));

    for path in &settings.exclude_dir {
        if file_path.contains(path) {
            return true;
        }
    }

    for name in &settings.exclude_file {
        if file_name.contains(name) {
            return true;
        }
    }

    return false;
}

fn is_hidden(file: &DirEntry) -> bool {
    return file
        .path()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .map(|s| return s.starts_with('.'))
        .unwrap_or(false);
}

pub fn walk(settings: &Settings) -> Vec<DirEntry> {
    let mut walker = WalkDir::new(&settings.root_dir).into_iter();
    let mut targets: Vec<DirEntry> = vec![];

    loop {
        let entry: DirEntry = match walker.next() {
            None => return targets,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(e)) => e,
        };

        if !is_hidden(&entry) && !is_excluded(&entry, settings) && !&entry.file_type().is_dir() {
            targets.push(entry);
        }
    }
}
