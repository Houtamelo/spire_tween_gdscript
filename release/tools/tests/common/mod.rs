use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
};

use sha2::{Digest, Sha256};

// Preserve valid checksums so failure controls exercise missing dependencies,
// rather than merely proving that integrity checks reject corrupt bytes.
pub fn without_file(source: &Path, destination: &Path, removed: &str) {
    let mut zip = zip::ZipArchive::new(fs::File::open(source).unwrap()).unwrap();
    let mut payload = BTreeMap::new();
    for index in 0..zip.len() {
        let mut file = zip.by_index(index).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        payload.insert(file.name().to_owned(), (data, file.unix_mode().unwrap_or(0o644)));
    }
    assert!(payload.remove(removed).is_some());
    payload.remove("SHA256SUMS");
    let sums = payload
        .iter()
        .map(|(name, (data, _))| format!("{:x}  {name}\n", Sha256::digest(data)))
        .collect::<String>();
    payload.insert("SHA256SUMS".into(), (sums.into_bytes(), 0o644));
    let mut writer = zip::ZipWriter::new(fs::File::create(destination).unwrap());
    for (name, (data, mode)) in payload {
        writer
            .start_file(name, zip::write::SimpleFileOptions::default().unix_permissions(mode))
            .unwrap();
        writer.write_all(&data).unwrap();
    }
    writer.finish().unwrap();
}
