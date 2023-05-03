use fs_err::File;
use std::{io::Read, path::Path};
use walkdir::WalkDir;

fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(Path::new(env!("CARGO_MANIFEST_DIR")).join(path)).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    file_content
}

#[test]
fn should_parse_webref_ed() {
    let mut entries = std::fs::read_dir("./tests/webref/ed/idl")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    // We sort entries to have deterministic test
    entries.sort();

    for path in &entries {
        println!("Name: {}", path.display());
        let content = read_file(path);
        weedle::parse(&content).unwrap();
    }
}

#[test]
fn should_parse_all_webref_idls() {
    let mut entries: Vec<_> = WalkDir::new("./tests/webref")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.into_path())
        .filter(|p| {
            p.extension().is_some() && p.extension().unwrap().to_string_lossy().contains("idl")
        })
        .collect();

    // We sort entries to have deterministic test
    entries.sort();

    for path in &entries {
        println!("Name: {}", path.display());
        let content = read_file(path);
        weedle::parse(&content).unwrap();
    }
}
