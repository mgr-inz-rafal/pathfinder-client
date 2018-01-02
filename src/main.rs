#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate iron;
extern crate bodyparser;
extern crate pathfinder;

use pathfinder::*;
use iron::prelude::*;
use iron::status;

// Share between pathfinder and pathfinder-server
#[derive(Deserialize)]
struct PathfinderRequest {
    pub width: u64,
    pub height: u64,
    pub start: (u64, u64),
    pub destination: (u64, u64),
    pub field: Vec<f64>
}

fn main() {
    Iron::new(
        |x: &mut Request| {
            let body = x.get::<bodyparser::Raw>();
            match body {
                Ok(Some(body)) => 
                {
                    // TODO: Decompose to methods
                    println!("Read body:\n{}\n", body);
                    let deserialized: PathfinderRequest = serde_json::from_str(&body).unwrap();
                    println!("Request deserialized correctly [JSON] - field total area: {}", deserialized.field.len());
                    println!("Invoking pathfinder lib...");
                    let result = calculate_shortest_path(
                        deserialized.width,
                        deserialized.height,
                        deserialized.field,
                        deserialized.start,
                        deserialized.destination);
                    println!("Calculated path: {}", result);
                    println!("Sending it back...");
                    return Ok(Response::with((status::Ok, result)));
                },
                Ok(None) => println!("No body"),
                Err(err) => println!("Error: {:?}", err)
            }
        Ok(Response::with((status::Ok, "Hello World!")))
        }
    ).http("localhost:3000").unwrap();

    /*
    let test_level = vec![
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 0.1, 0.7, 0.1, 0.1, 0.1, 1.0,
        1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
        1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
    ];
    let width = 7;
    let height = 5;
    let start = (1, 1);
    let destination = (4, 1);
    let result = calculate_shortest_path(width, height, test_level, start, destination);

    println!("Reversed path to destination: {}", result);
    */
}
