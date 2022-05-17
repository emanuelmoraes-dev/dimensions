pub mod ports;
pub mod scenarios;
pub mod operators;

use std::io;

fn main() -> Result<(), io::Error> {
    operators::cli::run()?;
    Ok(())
}
