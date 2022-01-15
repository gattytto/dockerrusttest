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

fn main() -> Result<(), Box<dyn Error>> {
    let key="KUBERNETES_SERVICE_HOST";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }
    //let res = reqwest::blocking::get("https://api.myip.com")?.json::<ApiRes>()?;
    Ok(thread::sleep(Duration::from_millis(100)))
    //println!("{}", res.ip);

    //Ok(())
}
