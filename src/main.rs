//use serde::Deserialize;
use std::error::Error;
use std::env;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

//#[derive(Deserialize, Debug)]
//struct ApiRes {
//    ip: String,
//}

use std::{
    thread,
    time::Duration,
};

use actix_web::{web, App, HttpServer, HttpResponse};

async fn get_health_status() -> HttpResponse {
    println!("liveness probe triggered");
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
//fn main() -> Result<(), Box<dyn Error>> {
    let key="KUBERNETES_SERVICE_HOST";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(get_health_status))
           // ^ Our new health route points to the get_health_status handler
    })
    .bind(("::", 8080))?
    .run()
    .await;
    //let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;
    Ok(thread::sleep(Duration::from_millis(500000)))
    //println!("{}", res.ip);

    //Ok(())
}
