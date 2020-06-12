use std::convert::Infallible;

use tokio::fs;
use warp::{Filter, Rejection, Reply};

use crate::health::Health;

mod health;

fn hello_world() -> impl Filter<Extract = (&'static str,), Error = Infallible> + Copy {
    warp::any().map(|| "Hello World")
}

fn health_check() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::path("health")
        .and_then(|| async {
            fs::read("health.txt")
                .await
                .map(|bytes| {
                    let contents = String::from_utf8_lossy(&bytes);
                    contents.parse::<Health>().unwrap_or(Health::Unknown)
                })
                .or_else(|_err| Result::<_, Infallible>::Ok(Health::Unknown))
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

fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::get().and(health_check().or(hello_world()))
}

#[cfg_attr(tarpaulin, skip)]
#[tokio::main]
async fn main() {
    let routes = routes();
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::routes;

    #[test]
    fn hello_world_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let filter = routes();

        let response = Runtime::new()?.block_on(warp::test::request().path("/").reply(&filter));
        let body = String::from_utf8(response.body().to_vec())?;

        assert_eq!(body, "Hello World");

        Ok(())
    }

    #[test]
    fn health_endpoint_returns_503_unknown_when_no_health_file()
    -> Result<(), Box<dyn std::error::Error>> {
        let filter = routes();

        let response =
            Runtime::new()?.block_on(warp::test::request().path("/health").reply(&filter));
        let body = String::from_utf8(response.body().to_vec())?;

        assert_eq!(response.status(), 503);
        assert_eq!(body, "Unknown");

        Ok(())
    }
}
