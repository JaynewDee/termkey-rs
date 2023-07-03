mod command;
mod encryption;
mod errors;

use std::process::exit;

use command::{control_flow, get_cl_args};
fn main() {
    handle_ctrlc();
    control_flow(get_cl_args()).expect("Fatal error @ control flow.");
    exit(0);
}

fn handle_ctrlc() {
    ctrlc::set_handler(|| {
        println!("Received EXIT event.");
        println!("Shutting down gracefully ... ");
        exit(0);
    })
    .expect("Error setting handler for Ctrl-C ...");
}
