extern crate docker;
extern crate cosmos;
extern crate rustc_serialize;

mod model;

use std::{env, thread};
use model::{Container, CosmosContainerDecodable};
use docker::Docker;
use cosmos::Cosmos;
use rustc_serialize::json;

fn run(host: &str, planet_name: &str) {
    let docker = Docker::new();
    let containers = match docker.get_containers(true) {
        Ok(containers) => containers,
        Err(e) => { println!("{}", e); return; }
    };

    let mut cosmos_containers: Vec<Container> = Vec::new();
    for container in containers.iter() {
        let stats = match docker.get_stats(&container) {
            Err(e) => { println!("{}", e); return; }
            Ok(stats) => stats
        };

        thread::sleep_ms(500);

        let delayed_stats = match docker.get_stats(&container) {
            Err(e) => { println!("{}", e); return; }
            Ok(stats) => stats
        };

        cosmos_containers.push(container.to_cosmos_container(&stats, &delayed_stats));
    }

    let encoded_cosmos_containers = json::encode(&cosmos_containers).unwrap();
    let cosmos = Cosmos::new(host);
    match cosmos.post_containers(planet_name, &encoded_cosmos_containers) {
        Ok(res) => {}
        Err(e) => { println!("{}", e); }
    };
}

fn main() {
    let host = match env::var("COSMOS_HOST") {
        Ok(host) => host,
        Err(_) => { panic!("COSMOS_HOST envrionment variable does not exist.") }
    };

    let planet_name = match env::var("COSMOS_PLANET_NAME") {
        Ok(planet_name) => planet_name,
        Err(_) => {
            let docker = Docker::new();
            let name = match docker.get_info() {
                Ok(info) => info.Name,
                Err(e) => { panic!("{}", e); }
            };
            name
        }
    };
    
    let interval: u32 = 5000; // ms, will be an option

    loop {
        run(&host, &planet_name);
        thread::sleep_ms(interval);
    }
}
