# Contributing

## Setup

1. Install [Git](https://git-scm.com/).
2. Install [Rust](https://rustup.rs/).
3. Clone the repository:

    ```bash
    git clone git@github.com:azriel91/myob_interview.git
    ```

4. For code coverage, also install [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) and [`cargo-nextest`](https://github.com/nextest-rs/nextest):

    ```bash
    rustup component add llvm-tools-preview
    cargo install cargo-llvm-cov
    cargo install cargo-nextest
    ```

## Testing

Tests are executed by running:

```bash
cargo test
```


## Coverage

Code coverage is collected by running:

```bash
cargo coverage
```

Collect coverage and open `html` report.

```bash
cargo coverage && cargo coverage_open
```
