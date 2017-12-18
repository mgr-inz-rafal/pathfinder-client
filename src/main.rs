extern crate pathfinder;

use pathfinder::*;

fn main() {
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
}
