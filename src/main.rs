#[allow(unused_imports)]
use std::fs;
use std::path::PathBuf;
// use std::ffi::OsString;
use walkdir::WalkDir;

fn main() {
    let freezes_dir = fs::create_dir_all(PathBuf::from("/Freezes"));
    //    let pth_freezes = PathBuf::from("/Freezes");
    let live_proj_freeze_path = "Samples/Processed/Freeze";

    // creates target dirs if needed

    let wav_files: Vec<PathBuf> = WalkDir::new("")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .to_str()
                .map_or(false, |s| s.contains(live_proj_freeze_path))
                || entry
                    .file_name()
                    .to_str()
                    .map_or(false, |s| s.ends_with(".wav"))
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    if !wav_files.is_empty() {
        for file_path in fs::read_dir("")? {
            let dir = entry?;
            println!("{:?}", dir.path());
            //             let destination = freezes_dir.expect("Couldn't load freezes_dir").clone();
            //             fs::copy(&file_path, );
        }
    } else {
        println!("No wav files found to copy");
    }
}
