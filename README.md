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

* Metadata endpoint:

    ```bash
    $ curl http://127.0.0.1:8000/metadata -s
    {"version":"0.1.1","description":"Web application with hello world, health, and metadata endpoints","last_commit_sha":"c3c69b5db198095c7dd4e975434181a092b761a3"}
    ```

    Formatted:

    ```json
    {
      "version": "0.1.1",
      "description": "Web application with hello world, health, and metadata endpoints",
      "last_commit_sha": "c3c69b5db198095c7dd4e975434181a092b761a3"
    }

    ```

### Contributing

For development instructions, please see the [contribution guide].

### Deploying

To deploy the `pett_server`, please see the [deployment guide].

### Maintenance Report

Information about this project's maintenance status can be found in the [maintenance report].

[contribution guide]: CONTRIBUTING.md
[deployment guide]: DEPLOYING.md
[maintenance report]: REPORT.md
[ops technical test]: https://github.com/MYOB-Technology/ops-technical-test
