# Report

## Assumptions

* Web application will be run on a 64-bit Linux or Windows host as a service.
* Logging is not required at this stage.
* Feedback of application state is not required at this stage.

## Maintenance Status

* **📦 Packaging:** Automated upload to GitHub releases on tag ([artifacts], [workflow][publish_workflow], [example][publish_example]).

    Includes binaries for:

    - Linux (64-bit)
    - Windows (64-bit)

* **⚙️ Continuous Integration:** Runs on push to all branches and tags ([workflow][ci_workflow], [example][ci_example]).

    Includes:

    - Dependency auditing
    - Code formatting
    - Linting
    - Native building and testing on Linux and Windows

## Issues / Opportunities

* Web server address is hard coded.

    Servers may have multiple network interfaces, and to allow a web application to bind to a desired network interface, the application should take in the address to bind to.

* Web server port is hard coded.

    In a locked down environment, web applications may be restricted to inbound traffic on certain ports. The web application should take in the port to listen on to accommodate such constraints.

    In addition, this disallows running multiple instances of `pett_server` at the same time.

* No feedback to indicate server status or usage.

    `pett_server` runs without logging any information. Logging is essential to discovering what happened during outages, and without this it becomes virtually impossible.

* No help built into the application.

    Documentation is important, but in outages, good built-in explanations are vital to restoring system operation as the operators addressing the outage may not have access to the documentation.

### Risks

* Network communication with the web server is in plain text, so it is visible to any packet analyzer.

    At this point there is no sensitive information transmitted between the `pett_server` and clients. However, it is best practice to build web applications to be secure by default through SSL.

* Responding to requests is not rate limited.

    `pett_server` does not explicitly rate-limit receiving and responding to requests. However, as it uses a single threaded asynchronous executor, there is no risk of CPU starvation.

    Adding rate-limiting support with explicit rejection responses provides backpressure to notify requestors to retry later.

[artifacts]: https://github.com/azriel91/myob_interview/releases
[ci_example]: https://github.com/azriel91/myob_interview/actions/runs/134912478
[ci_workflow]: https://github.com/azriel91/myob_interview/blob/master/.github/workflows/ci.yml
[publish_example]: https://github.com/azriel91/myob_interview/actions/runs/134913539
[publish_workflow]: https://github.com/azriel91/myob_interview/blob/master/.github/workflows/publish.yml
