pub mod builders;
pub mod configuration;
mod cpp;
mod rust;
mod util;

use configuration::Config;
use std::error::Error;

/// Generate bindings from a bindings configuration.
pub fn generate_bindings(config: &Config) -> Result<(), Box<dyn Error>> {
    rust::write_interface(config)?;

    cpp::write_header(config)?;
    cpp::write_cpp(config)?;
    Ok(())
}
