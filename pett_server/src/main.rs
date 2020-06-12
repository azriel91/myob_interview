use std::convert::Infallible;

use warp::Filter;

fn hello_world() -> impl Filter<Extract = (&'static str,), Error = Infallible> + Copy {
    warp::any().map(|| "Hello World")
}

#[tokio::main]
async fn main() {
    warp::serve(hello_world()).run(([127, 0, 0, 1], 8000)).await;
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::hello_world;

    #[test]
    fn hello_world_endpoint() -> Result<(), Box<dyn std::error::Error>> {
        let filter = hello_world();

        let value = Runtime::new()?.block_on(warp::test::request().path("/").filter(&filter))?;

        assert_eq!(value, "Hello World");

        Ok(())
    }
}
