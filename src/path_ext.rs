
use std::fs;
use std::io;
use std::path::Path;


pub(crate) trait PathExt {
    fn ensure_dir_exists(&self) -> io::Result<&Path>;
    fn ensure_is_writable(&self) -> io::Result<&Path>;
}

impl PathExt for Path {
    fn ensure_dir_exists(&self) -> io::Result<&Path> {
        if !self.is_dir() {
            fs::create_dir(&self)?;
        }
        Ok(self)
    }

    fn ensure_is_writable(&self) -> io::Result<&Path> {
        let metadata = self.metadata()?;
        if metadata.permissions().readonly() {
            Err(io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied"))
        } else {
            Ok(self)
        }
    }
}
