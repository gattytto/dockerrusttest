//use serde::Deserialize;
//use std::error::Error;
use std::env;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

//#[derive(Deserialize, Debug)]
//struct ApiRes {
//    ip: String,
//}

//use std::{
//    thread,
//    time::Duration,
//};

use actix_web::{web, App, HttpServer, HttpResponse};

async fn get_health_status() -> HttpResponse {
    println!("liveness probe triggered");
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Healthy!")
}

use pnet::datalink::interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
//fn main() -> Result<(), Box<dyn Error>> {
    let key="KUBERNETES_SERVICE_HOST";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
    
    // Get a vector with all network interfaces found
    let all_interfaces = interfaces();

// Search for the default interface - the one that is
// up, not loopback and has an IP.
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty() && !e.ips[1].to_string().contains("fe80"));
    let mut ip=String::new();
    match default_interface {
        Some(interface) => ip.push_str(&interface.ips[1].to_string()),
        None => println!("Error while finding the default interface."),
    }
    println!("bind");
    ip=ip.split("/").next().unwrap().to_string();
    let ipf="[".to_owned()+&ip.to_string()+&"]".to_owned()+&":8080".to_owned();
    println!("{}", ipf); 
    let _server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(get_health_status))
           // ^ Our new health route points to the get_health_status handler
    })
    .bind(ipf)?
    .run()
    .await;
    println!("bye");
    Ok(())
    //let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;
    //println!("{}", res.ip);

    //Ok(())
}
