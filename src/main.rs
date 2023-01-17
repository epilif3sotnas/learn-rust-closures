// internal
mod lambda;

use lambda::lambda::operation;

// external
#[macro_use]
extern crate log;
extern crate env_logger;


fn main () {
    env_logger::init();
    
    info!("Starting learn lambda in Rustlang");

    let sum = |a, b| {a + b};
    let sub = |a, b| {a - b};
    let mult = |a, b| {a * b};
    let div = |a, b| {a / b};

    info!("Sum result: {}", operation(2, 2, sum));
    info!("Sum result: {}", operation(2, 2, sub));
    info!("Sum result: {}", operation(2, 2, mult));
    info!("Sum result: {}", operation(2, 2, div));
}