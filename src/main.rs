use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    let env_url = env::var("DAEMON_URI");
    match env_url {
        Ok(_) => {
            let hello = warp::path("hello")
                .and(warp::path::param())
                .and(warp::header("user-agent"))
                .map(|param: String, agent: String| {
                    format!("Hello {}, whose agent is {}", param, agent)
                });

            warp::serve(hello)
                .run(([127, 0, 0, 1], 3030))
                .await;
        },
        Err(_) => panic!("Environment variable `DAEMON_URI` not provided.")
    }
}
