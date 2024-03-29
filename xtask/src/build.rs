mod allwinner_d1_series;
use allwinner_d1_series::{build_allwinner_d1_series, flash_allwinner_d1_series};
mod no_specific_platform;
use no_specific_platform::build_no_specific_platform;

use crate::{app::Platform, Config};
use std::error::Error;

pub fn build_main(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.platform {
        Platform::AllwinnerD1Series => build_allwinner_d1_series(config),
        Platform::Sophgo2002Series => todo!(),
        Platform::NoSpecificPlatform => build_no_specific_platform(config),
    }
    Ok(())
}

pub fn flash_main(config: &Config) -> Result<(), Box<dyn Error>> {
    match config.platform {
        Platform::AllwinnerD1Series => flash_allwinner_d1_series(),
        Platform::Sophgo2002Series => todo!(),
        Platform::NoSpecificPlatform => todo!(),
    }
    Ok(())
}
