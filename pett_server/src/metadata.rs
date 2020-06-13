use serde::{Deserialize, Serialize};

/// Application metadata at build time.
pub const METADATA: Metadata<'static> = Metadata {
    version: env!("CARGO_PKG_VERSION"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    last_commit_sha: env!("GIT_COMMIT_SHA"), // Set in `build.rs`
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata<'s> {
    /// Version of this application.
    pub version: &'s str,
    /// Human readable description of the purpose of this application.
    pub description: &'s str,
    /// SHA of the commit this application is built from.
    pub last_commit_sha: &'s str,
}
