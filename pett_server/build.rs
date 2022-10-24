use std::{
    env,
    error::Error,
    fmt::{self, Display},
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
};

use git2::{Oid, Repository};

#[derive(Debug)]
enum BuildError {
    MissingEnvVar(&'static str),
    RepositoryNotFound(git2::Error),
    HeadReferenceNotFound(git2::Error),
    CommitNotFound(git2::Error),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingEnvVar(env_var) => {
                write!(f, "The `{}` environmental variable is not set.", env_var)
            }
            Self::RepositoryNotFound(git2_error) => {
                write!(f, "The crate repository could not be found: {}", git2_error)
            }
            Self::HeadReferenceNotFound(git2_error) => write!(
                f,
                "The repository head reference could not be found: {}",
                git2_error
            ),
            Self::CommitNotFound(git2_error) => write!(
                f,
                "The repository head commit could not be found: {}",
                git2_error
            ),
        }
    }
}

impl Error for BuildError {}

fn content_differs(path: &Path, content: &str) -> Result<bool, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut current = String::new();
    f.read_to_string(&mut current)?;

    Ok(current != content)
}

fn latest_commit_sha() -> Result<Oid, BuildError> {
    let repository =
        Repository::discover(env!("CARGO_MANIFEST_DIR")).map_err(BuildError::RepositoryNotFound)?;
    let oid = repository
        .head()
        .map_err(BuildError::HeadReferenceNotFound)?
        .peel_to_commit()
        .map_err(BuildError::CommitNotFound)?
        .id();

    Ok(oid)
}

fn main() -> Result<(), Box<dyn Error>> {
    // The git commit may change even if we do not change Rust source files.
    //
    // This means we need to tell `cargo` to check the git history for the commit
    // value, then write it to a file.
    //
    // Setting an environment variable does not cause recompilation, so binaries not
    // affected by later git commits will not accurately store the git commit they
    // may be published from.
    let git_commit_sha = latest_commit_sha()?;

    let metadata_source_path =
        env::var_os("OUT_DIR").ok_or(BuildError::MissingEnvVar("OUT_DIR"))?;
    let metadata_source_path = Path::new(&metadata_source_path).join("git_metadata.rs");

    let metadata_source_content = format!(
        "\
        /// Git repository SHA that this application is published from.\n\
        pub const GIT_COMMIT_SHA: &str = \"{}\";\n\
        ",
        git_commit_sha
    );

    let should_write = if metadata_source_path.exists() {
        content_differs(&metadata_source_path, &metadata_source_content)?
    } else {
        true
    };

    if should_write {
        let mut file = BufWriter::new(File::create(&metadata_source_path)?);
        write!(file, "{}", metadata_source_content)?
    }

    Ok(())
}
