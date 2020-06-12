use std::error::Error;

use git2::{DescribeOptions, Repository};

fn commit_sha() -> Result<String, Box<dyn Error>> {
    let mut options = DescribeOptions::new();
    options.describe_tags().show_commit_oid_as_fallback(true);

    let repository = Repository::discover(env!("CARGO_MANIFEST_DIR"))?;
    let describe = repository.describe(&options)?;

    Ok(describe.format(None)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-env=GIT_COMMIT_SHA={}", commit_sha()?);

    Ok(())
}
