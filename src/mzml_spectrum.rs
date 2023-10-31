
#![allow(warnings, unused)]

use anyhow::*;
use quick_xml;
use serde::{Serialize, Deserialize};
use crate::mzml::*;

pub fn parse_mzml_spectrum_metadata(spectrum_header: &str) -> Result<MzMLSpectrumMetaData> {

    let parsed_mzml_header: MzMLSpectrumMetaData = quick_xml::de::from_str(spectrum_header)?;

    Ok(parsed_mzml_header)
}

// TODO: use the mzcore API when ready
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MzMLSpectrum {
    pub metadata: MzMLSpectrumMetaData,
    pub data: SpectrumData,
}

impl MzMLSpectrum {
    /// Creates a new spectrum
    ///
    /// # Arguments
    ///
    /// * `title` - Specturm title
    /// * `precursor_mz` -  Precursor mass
    /// * `precursor_charge` -  Precursor charge
    /// * `retention_time` -  Retention time
    /// * `mz_list` - M/Z list
    /// * `intensity_list` -  Intensity list
    ///
    pub fn new(
      metadata: MzMLSpectrumMetaData,
      data: SpectrumData,
    ) -> Self {
        Self {
            metadata,
            data,
        }
    }

    /// Returns M/Z list
    ///
    pub fn get_mz_list(&self) -> &Vec<f64> {
        &self.data.mz_list
    }

    /// Returns intensity list
    ///
    pub fn get_intensity_list(&self) -> &Vec<f64> {
        &self.data.intensity_list
    }

    pub fn get_ms_level(&self) -> u8 {
        let default_ms_level: u8 = if self.metadata.precursor_list.is_some() {2} else {1};

        self.metadata.cv_params.iter().find(|cvp| cvp.accession == MS_LEVEL_CV_ACCESSION).map(|cvp| {
             cvp.value.as_ref().map(|value| value.parse::<u8>().unwrap_or(default_ms_level) )
         }).flatten().unwrap_or(default_ms_level)
    }

    pub fn get_precursor_mz_and_charge(&self) -> (Option<f64>,Option<i8>) {
        let mut prec_mz_opt: Option<f64> = None;
        let mut prec_charge_opt: Option<i8> = None;

        let ms_level = self.get_ms_level();
        if ms_level > 1 && self.metadata.precursor_list.is_some() {
            let prec =  self.metadata.precursor_list.as_ref().unwrap().precursors.first().unwrap();

            let sel_ion_list = &prec.selected_ion_list;
            let first_sel_ion_cv_params = &sel_ion_list.selected_ions.first().unwrap().cv_params;
            prec_charge_opt = first_sel_ion_cv_params.iter()
                .find(|cv_param| cv_param.accession == CHARGE_STATE_CV_ACCESSION)
                .map(|cv_param| cv_param.value.as_ref().map(|value| value.parse::<i8>().unwrap_or(0 ))).flatten();

            let thermo_trailer_param_opt = self.metadata.scan_list.scans.first().map(|s| {
                s.user_params.iter().find(|user_param| user_param.name == "[Thermo Trailer Extra]Monoisotopic M/Z:")
            }).flatten();

            prec_mz_opt = thermo_trailer_param_opt.map(|trailer_param|  trailer_param.value.parse::<f64>().ok() ).flatten().or_else( || {
                first_sel_ion_cv_params.iter()
                    .find(|cv_param| cv_param.accession == SELECTED_ION_MZ_CV_ACCESSION)
                    .map(|cv_param| cv_param.value.as_ref().map(|value| value.parse::<f64>().unwrap() )).flatten()
            });
        }

        (prec_mz_opt, prec_charge_opt)
    }

    pub fn get_first_scan_start_time(&self) -> Option<f64> {
        self.metadata.scan_list.scans.first().map(|fs| {
            fs.cv_params.iter().find(|cvp| cvp.accession == SCAN_START_TIME_CV_ACCESSION).map(|start_time_cv| {
                start_time_cv.value.as_ref().map(|value| value.parse::<f64>().unwrap_or(0.0 ))
            }).flatten()
        }).flatten()
    }
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
    pub index: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultArrayLength")]
    pub default_array_length: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "scanList")]
    pub scan_list: ScanList,
    #[serde(rename = "precursorList")]
    pub precursor_list: Option<PrecursorList>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "scan", default)]
    pub scans: Vec<Scan>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Scan {
    #[serde(rename = "@instrumentConfigurationRef")]
    pub instrument_configuration_ref: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
    #[serde(rename = "userParam", default)]
    pub user_params: Vec<UserParam>,
    #[serde(rename = "scanWindowList")]
    pub scan_window_list: ScanWindowList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanWindowList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "scanWindow", default)]
    pub scan_windows: Vec<ScanWindow>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScanWindow {
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PrecursorList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "precursor", default)]
    pub precursors: Vec<Precursor>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Precursor {
    #[serde(rename = "@spectrumRef")]
    pub spectrum_ref: String,
    #[serde(rename = "isolationWindow")]
    pub isolation_window: Option<IsolationWindow>,
    #[serde(rename = "selectedIonList")]
    pub selected_ion_list: SelectedIonList,
    pub activation: Activation,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IsolationWindow {
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectedIonList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "selectedIon", default)]
    pub selected_ions: Vec<SelectedIon>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelectedIon {
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Activation {
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}
