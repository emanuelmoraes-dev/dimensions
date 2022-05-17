pub mod ports;
pub mod scenarios;
pub mod operators;

fn main() {
    operators::cli::run().unwrap();
}
