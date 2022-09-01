use actix_web::{get, web, App, HttpServer, Responder};
use core::panic;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct calculateDisselUsageRequest {
    distance: f32,
    yearOfProduction: i16,
    fuelUsagePer100KM: f32,
}
#[derive(Serialize)]
pub struct calculateDisselUsageResponse {
    fuelUsage: f32,
}

#[derive(Deserialize)]
pub struct probabilityOfUnitInjectorFailRequest {
    VIN: String,
}
#[derive(Serialize)]
pub struct probabilityOfUnitInjectorFailResponse {
    failProbability: String,
}

#[get("/calculateDisselUsageForDistance")]
async fn calculate_dissel_usage_for_distance(
    web::Query(params): web::Query<calculateDisselUsageRequest>,
) -> impl Responder {
    let distance = params.distance;
    let fuel_usage_per_100km = params.fuelUsagePer100KM;
    let result = calculateDisselUsageResponse {
        fuelUsage: distance * fuel_usage_per_100km / 100.0,
    };

    web::Json(result)
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probability_of_unit_injector_fail(
    web::Query(params): web::Query<probabilityOfUnitInjectorFailRequest>,
) -> impl Responder {
    let mut rng = rand::thread_rng();
    let result = probabilityOfUnitInjectorFailResponse {
        failProbability: String::from((rng.gen_range(0..100) as f32 / 100.0).to_string()),
    };
    web::Json(result)
}

fn get_port(args: Vec<String>) -> u16 {
    let mut port: u16 = 8080; //default port

    if args.len() > 1 {
        // checking if port was passed as an argument in command line
        match args[1].trim().parse::<u16>() {
            //checking if passed value is u16 type
            Ok(_ok) => {
                port = args[1].trim().parse().unwrap();
                println!(
                    "Server running on port {} (passed as command line argument)",
                    port
                );
            }
            Err(_e) => panic!("Passed port is not a valid u16 value. Provide correct number"),
        }
    } else {
        // port was not passed
        let port_variable_name = "SAT_API_PORT";
        match env::var(port_variable_name) {
            //checking if environment variable exists
            Ok(v) => match v.trim().parse::<u16>() {
                Ok(_ok) => { //checking if its valid u16 type
                    port = v.trim().parse().unwrap();
                    println!(
                        "Server running on port {} (extracted from environment variable)",
                        port
                    );
                }
                Err(_e) => panic!(
                    "{port_variable_name} doesn't contain valid u16 value. Provide correct number"
                ),
            },
            Err(_e) => println!("Server running on port {} (default)", port),
        }
    }
    port
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = get_port(args);
    HttpServer::new(|| {
        App::new()
            .service(calculate_dissel_usage_for_distance)
            .service(probability_of_unit_injector_fail)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
