use std::{convert::Infallible, path::Path};

use warp::{reply::Json, Filter, Rejection, Reply};

use crate::{
    application::Application, health::Health, health_checker::HealthChecker, metadata::METADATA,
};

mod application;
mod health;
mod health_checker;
mod metadata;

fn hello_world() -> impl Filter<Extract = (&'static str,), Error = Infallible> + Copy {
    warp::any().map(|| "Hello World")
}

fn health_check(
    health_checker: HealthChecker,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("health")
        .and_then(move || health_checker.check())
        .map(|health: Health| {
            let message = health.to_string();
            match health {
                Health::Ok | Health::Degraded => {
                    warp::reply::with_status(message, warp::http::StatusCode::OK)
                }
                Health::Down | Health::Unknown => {
                    warp::reply::with_status(message, warp::http::StatusCode::SERVICE_UNAVAILABLE)
                }
            }
        })
}

fn metadata() -> impl Filter<Extract = (Json,), Error = Rejection> + Copy {
    warp::path("metadata").map(|| warp::reply::json(&METADATA))
}

fn routes(
    base_directory: &Path,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let health_check = health_check(HealthChecker::new(base_directory));

    warp::get().and(health_check.or(metadata()).or(hello_world()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_directory = Application::root_dir()?;
    let routes = routes(&base_directory);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs::File, io::Write};

    use bytes::Bytes;
    use tempfile::TempDir;
    use tokio::runtime::Runtime;
    use warp::http::Response;

    use crate::{health_checker::HEALTH_TXT, metadata::Metadata, routes, Health};

    #[test]
    fn hello_world_endpoint_returns_200_hello_world() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/",
            health: Option::<String>::None,
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Hello World");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_200_ok_when_health_ok() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/health",
            health: Some(Health::Ok),
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Ok");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_200_degraded_when_health_degraded() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/health",
            health: Some(Health::Degraded),
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 200);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Degraded");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_503_down_when_health_down() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/health",
            health: Some(Health::Down),
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 503);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Down");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_503_unknown_when_no_health_file() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/health",
            health: Option::<String>::None,
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 503);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Unknown");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_503_unknown_when_health_file_invalid() -> Result<(), Box<dyn Error>>
    {
        let test_params = TestParams {
            endpoint: "/health",
            health: Some("invalid health"),
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 503);

        let body = String::from_utf8(response.body().to_vec())?;
        assert_eq!(body, "Unknown");

        Ok(())
    }

    #[test]
    fn metadata_endpoint_returns_200_with_metadata() -> Result<(), Box<dyn Error>> {
        let test_params = TestParams {
            endpoint: "/metadata",
            health: Some(Health::Ok),
        };

        let response = send_request(test_params)?;
        assert_eq!(response.status(), 200);

        let metadata = serde_json::from_slice::<'_, Metadata>(response.body())?;
        assert_eq!(metadata.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(
            metadata.description,
            "Web application with hello world, health, and metadata endpoints"
        );
        assert_eq!(metadata.last_commit_sha, crate::metadata::GIT_COMMIT_SHA);

        Ok(())
    }

    /// Sends a request to the specified endpoint and returns its response.
    ///
    /// This abstracts away the boilerplate for a test, so that each test only
    /// contains the input, execution, and assertion code.
    fn send_request<T>(test_params: TestParams<T>) -> Result<Response<Bytes>, Box<dyn Error>>
    where
        T: ToString,
    {
        // Decontruct the variable into fields
        let TestParams { endpoint, health } = test_params;
        let temp_dir = setup_base_directory(health)?;
        let routes = routes(temp_dir.path());

        // Actually send the request
        let response =
            Ok(Runtime::new()?.block_on(warp::test::request().path(endpoint).reply(&routes)));

        temp_dir.close()?;

        response
    }

    /// Returns the temporary directory to use for the test.
    ///
    /// If the test needs the health text file to be set up, it is created in
    /// the temporary directory.
    fn setup_base_directory<T>(health: Option<T>) -> Result<TempDir, Box<dyn Error>>
    where
        T: ToString,
    {
        let temp_dir = tempfile::tempdir()?;

        if let Some(health) = health {
            let health_txt_path = temp_dir.path().join(HEALTH_TXT);
            let mut file = File::create(health_txt_path)?;
            writeln!(file, "{}", health.to_string())?;
        }

        Ok(temp_dir)
    }

    struct TestParams<T> {
        /// Web application endpoint to request, such as "/health"
        endpoint: &'static str,
        /// Value to write into `health.txt`.
        health: Option<T>,
    }
}
