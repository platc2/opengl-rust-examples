use std::{ffi, fs, io};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil,
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self { Self::Io(other) }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    /// # Errors
    /// - Fail to get exe path
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Self, Error> {
        let exe_file_name = std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent()
            .ok_or(Error::FailedToGetExePath)?;

        Ok(Self { root_path: exe_path.join(rel_path) })
    }

    /// # Errors
    /// - Fail to get exe path
    /// - Fail to get file metadata
    /// - File contains 0x00
    /// # Panics
    /// - File too large
    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;
        let file_len = usize::try_from(file.metadata()?.len()).unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(file_len + 1);
        file.read_to_end(&mut buffer)?;
        if buffer.iter().any(|i| *i == 0) {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }

    /// # Errors
    /// - Fail to get exe path
    /// - Fail to get file metadata
    ///
    /// # Panics
    /// - File too large
    pub fn load_image(&self, resource_name: &str) -> Result<Vec<u8>, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;
        let file_len = usize::try_from(file.metadata()?.len()).unwrap();
        let mut buffer: Vec<u8> = Vec::with_capacity(file_len);
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();
    for part in location.split('/') {
        path = path.join(part);
    }
    path
}
