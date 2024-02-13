mod allwinner_d1_series;
use allwinner_d1_series::{build_allwinner_d1_series, flash_allwinner_d1_series};

use crate::{app::Platform, Config};
use std::error::Error;

pub fn build_main(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Some(platform) = &config.platform {
        match platform {
            Platform::AllwinnerD1Series => build_allwinner_d1_series(config),
            Platform::Sophgo2002Series => todo!(),
        }
    }
    Ok(())
}

pub fn flash_main(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Some(platform) = &config.platform {
        match platform {
            Platform::AllwinnerD1Series => flash_allwinner_d1_series(),
            Platform::Sophgo2002Series => todo!(),
        }
    }
    Ok(())
}
