use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{YgError, YgResult};

pub(crate) trait YgPath {
    fn yg_basedir(&self) -> YgResult<&Path>;
    fn yg_canonicalize(&self) -> YgResult<PathBuf>;
    fn yg_ensure_dir_exists(&self, context: String) -> YgResult<&Path>;
    fn yg_ensure_is_writable(&self, context: String) -> YgResult<&Path>;
}

impl YgPath for Path {
    fn yg_basedir(&self) -> YgResult<&Path> {
        self.parent().ok_or_else(|| {
            let msg = format!(
                "Unable to get base directory for \"{}\"",
                self.to_string_lossy().to_string()
            );
            YgError::io_error(msg)
        })
    }

    fn yg_canonicalize(&self) -> YgResult<PathBuf> {
        self.canonicalize().map_err(|e| {
            YgError::io_error_with_source(
                format!(
                    "Unable to get canonical path for \"{}\"",
                    self.to_string_lossy().to_string()
                ),
                e,
            )
        })
    }

    fn yg_ensure_dir_exists(&self, context: String) -> YgResult<&Path> {
        if !self.is_dir() {
            fs::create_dir(&self).map_err(|e| {
                YgError::io_error_with_source(
                    format!(
                        "[{}] Unable to create directory at \"{}\"",
                        context,
                        self.to_string_lossy().to_string()
                    ),
                    e,
                )
            })?;
        }
        Ok(self)
    }

    fn yg_ensure_is_writable(&self, context: String) -> YgResult<&Path> {
        let res = Command::new("test")
            .arg("-w")
            .arg(self.as_os_str())
            .status()
            .map_err(|e| {
                YgError::io_error_with_source(
                    format!(
                        "[{}] Failed to execute \"test -w {}\"",
                        context,
                        self.to_string_lossy().to_string()
                    ),
                    e,
                )
            })?;

        if res.success() {
            Ok(self)
        } else {
            let e = io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Permission denied",
            );
            Err(YgError::io_error_with_source(
                format!(
                    "[{}] Path \"{}\" is not writable",
                    context,
                    self.to_string_lossy().to_string()
                ),
                e,
            ))
        }
    }
}
