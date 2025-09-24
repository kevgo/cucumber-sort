mod prelude;

use camino::{Utf8Path, Utf8PathBuf};
use prelude::*;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            println!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<()> {
    // find all files in all subfolders that have the extension ".feature"
    let mut feature_files = vec![];
    find_feature_files(".", &mut feature_files)?;
    for file in feature_files {
        println!("Processing: {}", file);

        // read the content of the file into a Vec of lines.
        let file = fs::File::open(file).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        for (i, line) in lines.into_iter().enumerate() {
            let line = line.unwrap();
            println!("{}: {}", i + 1, line);
        }
        println!();
    }
    Ok(())
}

fn find_feature_files(dir: impl AsRef<Utf8Path>, files: &mut Vec<Utf8PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir.as_ref()).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let path = Utf8Path::from_path(&entry_path).unwrap();
        if path.is_dir() {
            find_feature_files(&path, files)?;
        } else if path.extension() == Some("feature") {
            files.push(path.to_path_buf());
        }
    }
    Ok(())
}
