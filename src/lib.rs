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
    pub fn from_osu_generic(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::osu::from_osu_generic(raw_chart)
    }

    #[inline]
    pub fn from_sm_generic(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::stepmania::from_sm_generic(raw_chart)
    }

    #[inline]
    pub fn from_qua_generic(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::quaver::from_qua_generic(raw_chart)
    }

    #[inline]
    pub fn from_fsc_generic(raw_chart: &str) -> Result<GenericManiaChart, Box<dyn Error>> {
        parsers::fluxis::from_fsc_generic(raw_chart)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod write {
    use crate::writers;
    use crate::GenericManiaChart;
    use std::error::Error;

    #[inline]
    pub fn to_osu_generic(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::osu::to_osu_generic(chart)
    }

    #[inline]
    pub fn to_sm_generic(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::stepmania::to_sm_generic(chart)
    }

    #[inline]
    pub fn to_qua_generic(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::quaver::to_qua_generic(chart)
    }

    #[inline]
    pub fn to_fsc_generic(chart: &GenericManiaChart) -> Result<String, Box<dyn Error>> {
        writers::fluxis::to_fsc_generic(chart)
    }
}

#[cfg(target_arch = "wasm32")]
pub mod parse {
    use wasm_bindgen::prelude::*;
    use crate::parsers;
    use crate::GenericManiaChart;

    #[wasm_bindgen(js_name = parseFromOsuGeneric)]
    pub fn parse_from_osu_generic(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::osu::from_osu_generic(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = parseFromSmGeneric)]
    pub fn parse_from_sm_generic(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::stepmania::from_sm_generic(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = parseFromQuaGeneric)]
    pub fn parse_from_qua_generic(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::quaver::from_qua_generic(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = parseFromFscGeneric)]
    pub fn parse_from_fsc_generic(raw_chart: &str) -> Result<GenericManiaChart, JsError> {
        parsers::fluxis::from_fsc_generic(raw_chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

#[cfg(target_arch = "wasm32")]
pub mod write {
    use wasm_bindgen::prelude::*;
    use crate::writers;
    use crate::GenericManiaChart;

    #[wasm_bindgen(js_name = writeToOsuGeneric)]
    pub fn write_to_osu_generic(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::osu::to_osu_generic(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = writeToSmGeneric)]
    pub fn write_to_sm_generic(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::stepmania::to_sm_generic(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = writeToQuaGeneric)]
    pub fn write_to_qua_generic(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::quaver::to_qua_generic(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = writeToFscGeneric)]
    pub fn write_to_fsc_generic(chart: &GenericManiaChart) -> Result<String, JsError> {
        writers::fluxis::to_fsc_generic(chart)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}
