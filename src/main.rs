use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // find all files in all subfolders that have the extension ".feature"
    let feature_files = find_feature_files(".")?;

    for file in feature_files {
        println!("Processing: {}", file);

        // read the content of the file into a Vec of lines.
        let lines = read_file_lines(&file)?;

        println!("File has {} lines", lines.len());
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
        println!();
    }

    Ok(())
}

fn find_feature_files(dir: impl AsRef<Utf8Path>) -> io::Result<Vec<Utf8PathBuf>> {
    let mut feature_files = Vec::new();
    let dir = dir.as_ref();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = Utf8PathBuf::from_path_buf(entry.path())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 path"))?;

        if path.is_dir() {
            let mut subdir_files = find_feature_files(&path)?;
            feature_files.append(&mut subdir_files);
        } else if path.extension() == Some("feature") {
            feature_files.push(path);
        }
    }

    Ok(feature_files)
}

fn read_file_lines(file_path: &Utf8Path) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}
