# Platform Enablement Technical Test

[![CI](https://github.com/azriel91/myob_interview/workflows/CI/badge.svg)](https://github.com/azriel91/myob_interview/actions?query=workflow%3ACI) [![codecov](https://codecov.io/gh/azriel91/myob_interview/branch/master/graph/badge.svg)](https://codecov.io/gh/azriel91/myob_interview)

MYOB [ops technical test] submission.

* "Hello World" endpoint:


    ```bash
    # Web application server
    $ cargo run --release
        Finished release [optimized] target(s) in 0.04s
         Running `target/release/pett_server`
    ```

    ```bash
    # Web client
    $ curl http://127.0.0.1:8000/
    Hello World
    ```

For development instructions, please see the [contribution guide].

[contribution guide]: CONTRIBUTING.md
[ops technical test]: https://github.com/MYOB-Technology/ops-technical-test
