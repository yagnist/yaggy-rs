
use std::fs;
use std::io;
use std::path::Path;
use std::rc::Rc;

use crate::{Result, Error};


pub(crate) trait YgPath {
    fn yg_ensure_dir_exists(&self, context: String) -> Result<&Path>;
    fn yg_ensure_is_writable(&self, context: String) -> Result<&Path>;
    fn yg_basedir(&self) -> Result<&Path>;
}

impl YgPath for Path {
    fn yg_ensure_dir_exists(&self, context: String) -> Result<&Path> {
        if !self.is_dir() {
            fs::create_dir(&self)
                .map_err(|e| Error::CreateDir { context: context, path: self.to_string_lossy().to_string(), source: e })?;
        }
        Ok(self)
    }

    fn yg_ensure_is_writable(&self, context: String) -> Result<&Path> {
        let metadata = self.metadata()?;
        if metadata.permissions().readonly() {
            let e = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
            Err(Error::NotWritable { context: context, path: self.to_string_lossy().to_string(), source: e })
        } else {
            Ok(self)
        }
    }

    fn yg_basedir(&self) -> Result<&Path> {
        self.parent()
            .ok_or(Error::Basedir { path: Rc::new(self.to_string_lossy().to_string()) })
    }
}
