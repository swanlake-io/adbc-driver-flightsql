//! Basic usage example for adbc-driver-flightsql
//!
//! This example demonstrates how to load the FlightSQL driver using the
//! automatically downloaded native library.

use adbc_core::options::AdbcVersion;
use adbc_core::Driver;
use adbc_driver_flightsql::DRIVER_PATH;
use adbc_driver_manager::ManagedDriver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the FlightSQL driver using the bundled library path
    let mut driver = ManagedDriver::load_dynamic_from_filename(
        DRIVER_PATH, // Just use the const - no need to hardcode paths!
        None,
        AdbcVersion::default(),
    )?;

    println!("Successfully loaded FlightSQL driver from: {}", DRIVER_PATH);

    // Create database handle
    let _db = driver.new_database()?;
    println!("Created database handle");

    // Create connection (this will fail without a real FlightSQL server)
    // Uncomment to test with a real server:
    // let mut conn = db.new_connection()?;
    // println!("Created connection");

    Ok(())
}
