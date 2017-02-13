#![allow(dead_code)]

use std::fs;

use walkdir::{WalkDir};

use types::*;

impl File {
    pub fn load(path: &Path) -> Result<File> {
        let mut f = fs::File::open(path)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;
        Ok(File {
            path: Rc::new(path.to_path_buf()),
            data: data,
        })
    }

    pub fn dump(&self) -> Result<()> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .open(self.path.as_path())?;
        f.set_len(self.data.len() as u64)?;
        f.write_all(&self.data)?;
        f.flush()?;
        Ok(())
    }
}


// FIXME: this should return a boxed iterator or something
pub fn load_paths(paths: &Vec<&Path>) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for p in paths {
        let files_iter = WalkDir::new(p).into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        for f in files_iter {
            files.push(File::load(f.path())?);
        }
    }
    Ok(files)
}

