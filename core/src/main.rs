pub mod ports;
pub mod act;
pub mod operators;

use std::io;

fn main() -> Result<(), io::Error> {
    operators::cli::run()?;
    Ok(())
}
