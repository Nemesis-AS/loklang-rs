use std::path::PathBuf;

mod metadata;
mod utils;

fn main() {
    let path: PathBuf = PathBuf::from("D:\\Music\\Test");

    let files: Vec<metadata::AudioMetadata> = utils::scan_dir(path);
    println!("Files: {:#?}", files);
}
