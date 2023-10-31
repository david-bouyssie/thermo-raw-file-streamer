#![allow(warnings, unused)]

use anyhow::*;
use quick_xml;
use serde::{Serialize, Deserialize};

pub const MS_LEVEL_CV_ACCESSION: &'static str = "MS:1000511";
pub const CHARGE_STATE_CV_ACCESSION: &'static str = "MS:1000041";
pub const SCAN_START_TIME_CV_ACCESSION: &'static str = "MS:1000016";
pub const SELECTED_ION_MZ_CV_ACCESSION: &'static str = "MS:1000744";

pub fn parse_mzml_metadata(mzml_header: &str) -> Result<MzMLMetaData> {

    let parsed_mzml_header: MzMLMetaData = quick_xml::de::from_str(mzml_header)?;

    Ok(parsed_mzml_header)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MzMLMetaData {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "cvList")]
    pub cv_list: CvList,
    #[serde(rename = "fileDescription")]
    pub file_description: FileDescription,
    #[serde(rename = "referenceableParamGroupList")]
    pub referenceable_param_group_list: ReferenceableParamGroupList,
    #[serde(rename = "sampleList")]
    pub sample_list: SampleList,
    #[serde(rename = "softwareList")]
    pub software_list: SoftwareList,
    #[serde(rename = "instrumentConfigurationList")]
    pub instrument_configuration_list: InstrumentConfigurationList,
    #[serde(rename = "dataProcessingList")]
    pub data_processing_list: DataProcessingList,
    pub run: Run,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CvParam {
    #[serde(rename = "@cvRef")]
    pub cv_ref: String,
    #[serde(rename = "@accession")]
    pub accession: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@unitCvRef")]
    pub unit_cv_ref: Option<String>,
    #[serde(rename = "@unitAccession")]
    pub unit_accession: Option<String>,
    #[serde(rename = "@unitName")]
    pub unit_name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@type")]
    pub r#type: String,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "cv", default)]
    pub cv_entries: Vec<Cv>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cv {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fullName")]
    pub full_name: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@URI")]
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileDescription {
    #[serde(rename = "fileContent")]
    pub file_content: FileContent,
    #[serde(rename = "sourceFileList")]
    pub source_file_list: SourceFileList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceFileList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "sourceFile", default)]
    pub source_files: Vec<SourceFile>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceFile {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@location")]
    pub location: String,
    #[serde(rename = "cvParam",default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReferenceableParamGroupList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "referenceableParamGroup", default)]
    pub referenceable_param_groups: Vec<ReferenceableParamGroup>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReferenceableParamGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SampleList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "sample", default)]
    pub samples: Vec<Sample>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Sample {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SoftwareList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "software", default)]
    pub software_entries: Vec<Software>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Software {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InstrumentConfigurationList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "instrumentConfiguration", default)]
    pub instrument_configurations: Vec<InstrumentConfiguration>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstrumentConfiguration {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: ReferenceableParamGroupRef,
    #[serde(rename = "componentList")]
    pub component_list: ComponentList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupRef {
    #[serde(rename = "@ref")]
    pub r#ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentList {
    #[serde(rename = "@count")]
    pub count: String,
    pub source: Option<Source>,
    pub analyzer:  Option<Analyzer>,
    pub detector:  Option<Detector>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@order")]
    pub order: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analyzer {
    #[serde(rename = "@order")]
    pub order: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detector {
    #[serde(rename = "@order")]
    pub order: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataProcessingList {
    #[serde(rename = "@count")]
    pub count: String,
    #[serde(rename = "dataProcessing", default)]
    pub data_processings: Vec<DataProcessing>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataProcessing {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "processingMethod")]
    pub processing_method: ProcessingMethod,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProcessingMethod {
    #[serde(rename = "@order")]
    pub order: String,
    #[serde(rename = "@softwareRef")]
    pub software_ref: String,
    #[serde(rename = "cvParam", default)]
    pub cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    pub default_instrument_configuration_ref: String,
    #[serde(rename = "@startTimeStamp")]
    pub start_time_stamp: String,
    #[serde(rename = "@defaultSourceFileRef")]
    pub default_source_file_ref: String,
    #[serde(rename = "@sampleRef")]
    pub sample_ref: String,
}

