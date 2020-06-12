# Platform Enablement Technical Test

[![CI](https://github.com/azriel91/myob_interview/workflows/CI/badge.svg)](https://github.com/azriel91/myob_interview/actions?query=workflow%3ACI) [![codecov](https://codecov.io/gh/azriel91/myob_interview/branch/master/graph/badge.svg)](https://codecov.io/gh/azriel91/myob_interview)

MYOB [ops technical test] submission.

## Pett Server

The server binds to `127.0.0.1:8000` when run:

```bash
# Web application server
$ ./pett_server
```

The following shows the output when accessing the available endpoints:

* "Hello World" endpoint:

    ```bash
    # Web client
    $ curl http://127.0.0.1:8000/
    Hello World
    ```

* Health endpoint:

    Depending on the value in `health.txt`, the server returns a different health status.

    ```bash
    # Client requests
    for health in ok degraded down unknown invalid
    do
        echo $health > ./pett_server/health.txt
        curl http://127.0.0.1:8000/health -w "\n%{http_code} " -s | tac
    done

    200 Ok
    200 Degraded
    503 Down
    503 Unknown
    503 Unknown
    ```

For development instructions, please see the [contribution guide].

[contribution guide]: CONTRIBUTING.md
[ops technical test]: https://github.com/MYOB-Technology/ops-technical-test
