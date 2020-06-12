# Contributing

## Setup

1. Install [Git](https://git-scm.com/).
2. Install [Rust](https://rustup.rs/).
3. Clone the repository:

    ```bash
    git clone git@github.com:azriel91/myob_interview.git
    ```

4. For code coverage, also install [`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin):

    ```bash
    # Only supported on 64-bit Linux
    cargo install cargo-tarpaulin
    ```

## Testing

Tests are executed by running:

```bash
cargo test
```

Code coverage is collected by running:

```bash
cargo tarpaulin
```

Coverage is viewable by opening `target/tarpaulin/tarpaulin-report.html`.
