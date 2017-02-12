#![allow(dead_code)]

use std::fs;

use types::*;

impl<'a> File<'a> {
    fn load(path: &'a Path) -> Result<File<'a>> {
        let mut f = fs::File::open(path)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;
        Ok(File {
            path: path,
            data: data,
        })
    }

    fn dump(&self) -> Result<()> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .open(self.path)?;
        f.set_len(self.data.len() as u64)?;
        f.write_all(&self.data)?;
        f.flush()?;
        Ok(())
    }
}

fn load_files<'a>(paths: Vec<&'a Path>) -> Result<Vec<File<'a>>> {
    let mut files = Vec::new();
    for p in paths {
        files.push(File::load(p)?);
    }
    Ok(files)
}

