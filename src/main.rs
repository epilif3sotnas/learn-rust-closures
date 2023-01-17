// internal
mod lambda;

use lambda::lambda::{operation, something, logging_levels};

// external
#[macro_use]
extern crate log;
extern crate env_logger;


fn logging () {
    trace!("TRACE");
    debug!("DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("ERROR");
}

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

    let print_hello_world = || println!("Hello World!");
    print_hello_world();

    let print_log = || debug!("This is a log with level DEBUG");
    print_log();

    let calling_no_params = || {
        let a = 10 * 10;
        println!("Result: {}", a);
    };
    something(calling_no_params);

   
    let levels = || {
        trace!("TRACE");
        debug!("DEBUG");
        info!("INFO");
        warn!("WARN");
        error!("ERROR");
    };

    logging_levels(levels);
    logging_levels(logging);
}