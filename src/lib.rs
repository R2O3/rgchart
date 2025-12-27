use wasm_bindgen::prelude::*;

pub mod errors;
pub(crate) mod models;
pub(crate) mod utils;

#[macro_use]
pub(crate) mod macros;

mod parsers;
mod writers;

pub use models::common;
pub use models::generic;
pub use models::fluxis;
pub use models::osu;
pub use models::quaver;

pub use generic::GenericManiaChart;
pub use fluxis::FscFile;
pub use osu::OsuFile;
pub use quaver::QuaFile;
pub use common::{GameMode, KeyType, Key};

#[cfg(not(target_arch = "wasm32"))]
pub mod parse {
    use crate::parsers;
    use crate::GenericManiaChart;
    use std::error::Error;

    #[inline]
    pub fn from_osu(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::osu::from_osu(raw_chart)
    }

    #[inline]
    pub fn from_sm(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::stepmania::from_sm(raw_chart)
    }

    #[inline]
    pub fn from_qua(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::quaver::from_qua(raw_chart)
    }

    #[inline]
    pub fn from_fsc(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::fluxis::from_fsc(raw_chart)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod write {
    use crate::writers;
    use crate::GenericManiaChart;
    use std::error::Error;

    #[inline]
    pub fn to_osu(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::osu::to_osu(chart)
    }

    #[inline]
    pub fn to_sm(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::stepmania::to_sm(chart)
    }

    #[inline]
    pub fn to_qua(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::quaver::to_qua(chart)
    }

    #[inline]
    pub fn to_fsc(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::fluxis::to_fsc(chart)
    }
}

#[cfg(target_arch = "wasm32")]
pub mod parse {
    use wasm_bindgen::prelude::*;
    use crate::parsers;
    use crate::GenericManiaChart;

    #[wasm_bindgen]
    pub fn parse_from_osu(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::osu::from_osu(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn parse_from_sm(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::stepmania::from_sm(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn parse_from_qua(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::quaver::from_qua(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn parse_from_fsc(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::fluxis::from_fsc(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[cfg(target_arch = "wasm32")]
pub mod write {
    use wasm_bindgen::prelude::*;
    use crate::writers;
    use crate::GenericManiaChart;

    #[wasm_bindgen]
    pub fn write_to_osu(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::osu::to_osu(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn write_to_sm(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::stepmania::to_sm(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn write_to_qua(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::quaver::to_qua(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn write_to_fsc(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::fluxis::to_fsc(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
