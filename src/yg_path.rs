use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{YgIoError, YgIoResult};

pub(crate) trait YgPath {
    fn yg_basedir(&self) -> YgIoResult<&Path>;
    fn yg_canonicalize(&self) -> YgIoResult<PathBuf>;
    fn yg_ensure_dir_exists(&self, context: &str) -> YgIoResult<&Path>;
    fn yg_ensure_is_writable(&self, context: &str) -> YgIoResult<&Path>;
}

impl YgPath for Path {
    fn yg_basedir(&self) -> YgIoResult<&Path> {
        self.parent().ok_or_else(|| {
            YgIoError::BaseDirError(self.to_string_lossy().to_string())
        })
    }

    fn yg_canonicalize(&self) -> YgIoResult<PathBuf> {
        self.canonicalize().map_err(|err| {
            YgIoError::CanonicalPathError(
                self.to_string_lossy().to_string(),
                err,
            )
        })
    }

    fn yg_ensure_dir_exists(&self, context: &str) -> YgIoResult<&Path> {
        if !self.is_dir() {
            fs::create_dir(&self).map_err(|err| {
                YgIoError::CreateDirError(
                    context.to_string(),
                    self.to_string_lossy().to_string(),
                    err,
                )
            })?;
        }
        Ok(self)
    }

    fn yg_ensure_is_writable(&self, context: &str) -> YgIoResult<&Path> {
        let res = Command::new("test")
            .arg("-w")
            .arg(self.as_os_str())
            .status()
            .map_err(|err| {
                YgIoError::ExecError(
                    context.to_string(),
                    format!(
                        "test -w \"{}\"",
                        self.to_string_lossy().to_string()
                    ),
                    err,
                )
            })?;

        if res.success() {
            Ok(self)
        } else {
            let err = io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Permission denied",
            );
            Err(YgIoError::NotWritableError(
                context.to_string(),
                self.to_string_lossy().to_string(),
                err,
            ))
        }
    }
}
