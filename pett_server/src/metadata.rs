/// Application metadata at build time.
pub const _METADATA: Metadata = Metadata {
    version: env!("CARGO_PKG_VERSION"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    last_commit_sha: env!("GIT_COMMIT_SHA"), // Set in `build.rs`
};

#[derive(Debug)]
pub struct Metadata {
    /// Version of this application.
    pub version: &'static str,
    /// Human readable description of the purpose of this application.
    pub description: &'static str,
    /// SHA of the commit this application is built from.
    pub last_commit_sha: &'static str,
}
