use actix_web::http::Error;
use core::future::Future;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use core::time::Duration;
use env_logger::Env;
use log::debug;

use uuid::Uuid;

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HealthCheck {
    pub project: String,
    pub time: u128,
}

#[get("/")]
async fn hello() -> impl Responder
// -> impl Responder
{
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let req_uuid = Uuid::new_v4();

    debug!("before block: {}", req_uuid);

    // imagine calculating the health check,
    // that it took some awful 5-second hard loop time of calculation to find the "answer"
    // and then return -- this thread level sleep call will simulate this hard loop time
    // wrapping this in web::block allows this "work" to be put onto another
    // separate thread, and block returns a future, so it can be awaited at the worker level
    // leaving it to service and queue other requests that come in
    let health = web::block(move || {
        debug!("before sleep: {}", req_uuid);

        std::thread::sleep(Duration::from_secs(5)); // imaginary hard work!

        debug!("after sleep: {}", req_uuid);

        let health = HealthCheck {
            project: "actix-web-playground".to_string(),
            time,
        };

        // this is weird; block returns a result, and rust yells that it doesn't _know_ what the
        // types of err is, without this
        if false {
            return Err(());
        }

        // returns out "answer" -- which is just a plain health check
        Ok(health)
    })
    .await
    .unwrap(); // unwrap will extract the Ok value, less work for us
               // but if that didn't work, then it will all just crash, no error handling for us (right now)

    // std::thread::sleep(Duration::from_secs(5)); // only allows as many workers at once, which is useless
    // tokio::time::sleep(Duration::from_secs(5)).await;
    // broken because tokio 1.5.0 isn't supported yet, sleep doesn't exist on 0.2 and even if you try the compat option it fails
    // if we didn't have this cool web::block above, then this would be how to simulate this
    // but it would be a poor simulation since it would not actually do "work"
    // off this worker thread
    // tokio::time::delay_for(Duration::from_secs(5)).await; // this works the best so far

    debug!("after block: {}", req_uuid);

    // let health = HealthCheck {
    //     project: "actix-web-playground".to_string(),
    //     time,
    // };

    // return to the client
    HttpResponse::Ok().json(health)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let workers = 4; // workers map to core count by default, but we could add more
    let bind_address = "127.0.0.1:8080"; // bind address and/or port

    println!("ok, the server is alive: http://{}/", bind_address);

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(hello))
        .workers(workers)
        .bind(bind_address)?
        .run()
        .await
}
