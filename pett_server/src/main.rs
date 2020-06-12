use std::{convert::Infallible, path::Path, sync::Arc};

use tokio::fs;
use warp::{Filter, Rejection, Reply};

use crate::{application::Application, health::Health};

mod application;
mod health;

fn hello_world() -> impl Filter<Extract = (&'static str,), Error = Infallible> + Copy {
    warp::any().map(|| "Hello World")
}

fn health_check(
    base_directory: &Path,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let health_file = Arc::new(base_directory.join("health.txt"));

    warp::path("health")
        .and_then(move || {
            let health_file = Arc::clone(&health_file);
            async move {
                let health_file = health_file.as_path();
                fs::read(health_file)
                    .await
                    .map(|bytes| {
                        let contents = String::from_utf8_lossy(&bytes);
                        contents.parse::<Health>().unwrap_or(Health::Unknown)
                    })
                    .or_else(|_err| Result::<_, Infallible>::Ok(Health::Unknown))
            }
        })
        .map(|health| {
            let message = format!("{}", health);

            match health {
                Health::Ok | Health::Degraded | Health::Down => {
                    warp::reply::with_status(message, warp::http::StatusCode::OK)
                }
                Health::Unknown => {
                    warp::reply::with_status(message, warp::http::StatusCode::SERVICE_UNAVAILABLE)
                }
            }
        })
}

fn routes(
    base_directory: &Path,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and(health_check(base_directory).or(hello_world()))
}

#[cfg_attr(tarpaulin, skip)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_directory = Application::root_dir()?;
    let routes = routes(&base_directory);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::{routes, Application};

    #[test]
    fn hello_world_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let filter = routes(&Application::root_dir()?);

        let response = Runtime::new()?.block_on(warp::test::request().path("/").reply(&filter));
        let body = String::from_utf8(response.body().to_vec())?;

        assert_eq!(body, "Hello World");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_503_unknown_when_no_health_file()
    -> Result<(), Box<dyn std::error::Error>> {
        let filter = routes(&Application::root_dir()?);

        let response =
            Runtime::new()?.block_on(warp::test::request().path("/health").reply(&filter));
        let body = String::from_utf8(response.body().to_vec())?;

        assert_eq!(response.status(), 503);
        assert_eq!(body, "Unknown");

        Ok(())
    }
}
