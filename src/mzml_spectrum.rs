
#![allow(warnings, unused)]

use anyhow::*;
use quick_xml;
use serde::{Serialize, Deserialize};
use crate::mzml::{CvParam, UserParam};

pub fn parse_mzml_spectrum_header(spectrum_header: &str) -> Result<MzMLSpectrumMetaData> {

    let parsed_mzml_header: MzMLSpectrumMetaData = quick_xml::de::from_str(spectrum_header)?;

    Ok(parsed_mzml_header)
}

// TODO: use the mzcore API when ready
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SpectrumData {
    pub mz_list: Vec<f64>,
    pub intensity_list: Vec<f64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MzMLSpectrumMetaData {
    #[serde(rename = "@index")]
    index: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@defaultArrayLength")]
    default_array_length: String,
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
    #[serde(rename = "scanList")]
    scan_list: ScanList,
    #[serde(rename = "precursorList")]
    precursor_list: Option<PrecursorList>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
    scan: Scan,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Scan {
    #[serde(rename = "@instrumentConfigurationRef")]
    instrument_configuration_ref: String,
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    user_params: Vec<UserParam>,
    #[serde(rename = "scanWindowList")]
    scan_window_list: ScanWindowList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanWindowList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "scanWindow")]
    scan_window: ScanWindow,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanWindow {
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    count: String,
    precursor: Precursor,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Precursor {
    #[serde(rename = "@spectrumRef")]
    spectrum_ref: String,
    isolation_window: Option<IsolationWindow>,
    #[serde(rename = "selectedIonList")]
    selected_ion_list: SelectedIonList,
    activation: Activation,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IsolationWindow {
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectedIonList {
    #[serde(rename = "@count")]
    count: String,
    selected_ion: Option<SelectedIon>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectedIon {
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Activation {
    #[serde(rename = "cvParam", default)]
    cv_params: Vec<CvParam>,
}
