use std::{fs::read_to_string, path::Path};

use crate::{error::Error, prelude::Result};

pub struct FileLoader {
    path: String,
}

impl FileLoader {
    pub fn new(path: &str) -> FileLoader
    {
        FileLoader{path: path.trim().to_string()}
    }

    pub fn get_file_content(&mut self) -> Result<String>
    {
        let path = Path::new(&self.path);
        if !path.exists() || path.is_dir()
        {
            Err(Error::Generic("Could not load file, because it does not exist or is an directory".to_string()))?
        }

        let return_data = match read_to_string(path) {
            Ok(data) => data,
            Err(err) => Err(Error::Generic(err.to_string()))?
        };
        
        Ok(return_data)
    }

    pub fn get_file_lines(&mut self) -> Result<Vec<String>> {
        let data = match self.get_file_content() {
            Ok(data) => data,
            Err(err) => Err(Error::Generic(err.to_string()))?,
        };
        let lines = data.lines().into_iter().map(|line| line.to_string()).collect();
        Ok(lines)
    }
}