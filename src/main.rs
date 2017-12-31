extern crate iron;
extern crate bodyparser;
extern crate pathfinder;

use pathfinder::*;
use iron::prelude::*;
use iron::status;


fn main() {
    Iron::new(|x: &mut Request| {
    let body = x.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();
    
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
