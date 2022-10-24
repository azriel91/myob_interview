use std::{
    convert::Infallible,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::fs;

use crate::Health;

/// Name of the file to read application health from.
pub const HEALTH_TXT: &str = "health.txt";

#[derive(Clone, Debug)]
pub(crate) struct HealthChecker {
    /// Path to the file to read server health from.
    health_file: Arc<PathBuf>,
}

impl HealthChecker {
    pub(crate) fn new(base_directory: &Path) -> Self {
        let health_file = Arc::new(base_directory.join(HEALTH_TXT));

        Self { health_file }
    }

    /// Returns a `Future` that resolves to the server's `Health`.
    pub(crate) fn check(&self) -> impl Future<Output = Result<Health, Infallible>> {
        let health_file = Arc::clone(&self.health_file);
        async move {
            let health_file = health_file.as_path();
            fs::read(health_file)
                .await
                .map(|bytes| {
                    let contents = String::from_utf8_lossy(&bytes);
                    contents.parse::<Health>().unwrap_or(Health::Unknown)
                })
                .or(Result::<_, Infallible>::Ok(Health::Unknown))
        }
    }
}
