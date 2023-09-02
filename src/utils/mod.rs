use std::{fs, path::PathBuf};

use crate::metadata;

pub fn scan_dir(dir_path: PathBuf) -> Vec<metadata::AudioMetadata> {
    let mut info: Vec<metadata::AudioMetadata> = vec![];

    if let Ok(items) = fs::read_dir(dir_path) {
        for item in items {
            match item {
                Ok(entry) => {
                    let p: PathBuf = entry.path();

                    if p.is_dir() {
                        println!("Found Dir: {}", p.display());
                        let mut dir_info = scan_dir(p);
                        info.append(&mut dir_info);
                    } else {
                        println!("Scanning File: {}", p.display());
                        if let Some(meta) = metadata::get_metadata(p) {
                            info.push(meta);
                        }
                    }
                }
                Err(err) => {
                    println!("An Error Occurred while reading file!\n{:?}", err);
                    continue;
                }
            }
        }
    }

    info
}
