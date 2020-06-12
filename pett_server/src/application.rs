use std::{env, io, path::PathBuf};

pub(crate) struct Application;

impl Application {
    /// Find the effective working directory for the current application.
    ///
    /// During development, this is the crate's directory, read from
    /// `CARGO_MANIFEST_DIR`. In release, this is the directory containing
    /// the executable.
    pub(crate) fn root_dir() -> Result<PathBuf, io::Error> {
        if let Some(manifest_dir) = env::var_os("CARGO_MANIFEST_DIR") {
            return Ok(PathBuf::from(manifest_dir));
        }

        let mut exe_path = env::current_exe()?;

        // Modify in-place to avoid an extra copy.
        if exe_path.pop() {
            return Ok(exe_path);
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to find an application root",
        ))
    }
}
