#![allow(warnings, unused)]

use anyhow::*;
use quick_xml;
use serde::{Serialize, Deserialize};

pub fn parse_mzml_header(mzml_header: &str) -> Result<MzMLMetaData> {

    let parsed_mzml_header: MzMLMetaData = quick_xml::de::from_str(mzml_header)?;

    Ok(parsed_mzml_header)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MzMLMetaData {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "cvList")]
    cv_list: CvList,
    #[serde(rename = "fileDescription")]
    file_description: FileDescription,
    #[serde(rename = "referenceableParamGroupList")]
    referenceable_param_group_list: ReferenceableParamGroupList,
    #[serde(rename = "sampleList")]
    sample_list: SampleList,
    #[serde(rename = "softwareList")]
    software_list: SoftwareList,
    #[serde(rename = "instrumentConfigurationList")]
    instrument_configuration_list: InstrumentConfigurationList,
    #[serde(rename = "dataProcessingList")]
    data_processing_list: DataProcessingList,
    run: Run,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CvParam {
    #[serde(rename = "@cvRef")]
    cv_ref: String,
    #[serde(rename = "@accession")]
    accession: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: Option<String>,
    #[serde(rename = "@unitCvRef")]
    unit_cv_ref: Option<String>,
    #[serde(rename = "@unitAccession")]
    unit_accession: Option<String>,
    #[serde(rename = "@unitName")]
    unit_name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserParam {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@type")]
    r#type: String,
}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CvList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "cv", default)]
    cvs: Vec<Cv>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cv {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@fullName")]
    full_name: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@URI")]
    uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileDescription {
    #[serde(rename = "fileContent")]
    file_content: FileContent,
    #[serde(rename = "sourceFileList")]
    source_file_list: SourceFileList,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceFileList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "source_file", default)]
    source_files: Vec<SourceFile>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceFile {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@location")]
    location: String,
    #[serde(rename = "cv_param",default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReferenceableParamGroupList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "referenceable_param_group", default)]
    referenceable_param_groups: Vec<ReferenceableParamGroup>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReferenceableParamGroup {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SampleList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "sample", default)]
    samples: Vec<Sample>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Sample {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SoftwareList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "software", default)]
    software_entries: Vec<Software>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Software {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InstrumentConfigurationList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "instrument_configuration", default)]
    instrument_configurations: Vec<InstrumentConfiguration>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstrumentConfiguration {
    #[serde(rename = "@id")]
    id: String,
    referenceable_param_group_ref: ReferenceableParamGroupRef,
    component_list: ComponentList,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceableParamGroupRef {
    #[serde(rename = "@ref")]
    r#ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentList {
    #[serde(rename = "@count")]
    count: String,
    source: Source,
    analyzer: Analyzer,
    detector: Detector,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@order")]
    order: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analyzer {
    #[serde(rename = "@order")]
    order: String,
    cv_param: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detector {
    #[serde(rename = "@order")]
    order: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataProcessingList {
    #[serde(rename = "@count")]
    count: String,
    #[serde(rename = "data_processing", default)]
    data_processings: Vec<DataProcessing>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataProcessing {
    #[serde(rename = "@id")]
    id: String,
    processing_method: ProcessingMethod,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProcessingMethod {
    #[serde(rename = "@order")]
    order: String,
    #[serde(rename = "@softwareRef")]
    software_ref: String,
    #[serde(rename = "cv_param", default)]
    cv_params: Vec<CvParam>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@defaultInstrumentConfigurationRef")]
    default_instrument_configuration_ref: String,
    #[serde(rename = "@startTimeStamp")]
    start_time_stamp: String,
    #[serde(rename = "@defaultSourceFileRef")]
    default_source_file_ref: String,
    #[serde(rename = "@sampleRef")]
    sample_ref: String,
}


    /*
    let s2 = r#"<?xml version="1.0" encoding="utf-8"?>
<mzML xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://psi.hupo.org/ms/mzml http://psidev.info/files/ms/mzML/xsd/mzML1.1.0.xsd" version="1.1.0" id="smal
l" xmlns="http://psi.hupo.org/ms/mzml">
  <cvList count="2">
    <cv id="MS" fullName="Mass spectrometry ontology" version="4.1.12" URI="https://raw.githubusercontent.com/HUPO-PSI/psi-ms-CV/master/psi-ms.obo" />
    <cv id="UO" fullName="Unit Ontology" version="09:04:2014" URI="https://raw.githubusercontent.com/bio-ontology-research-group/unit-ontology/master/unit.obo" />
  </cvList>
  <fileDescription>
    <fileContent>
      <cvParam cvRef="MS" accession="MS:1000579" value="" name="MS1 spectrum" />
      <cvParam cvRef="MS" accession="MS:1000580" value="" name="MSn spectrum" />
    </fileContent>
    <sourceFileList count="1">
      <sourceFile id="RAW1" name="small" location="file:///D:\Dev\rust\rusteomics\thermo-raw-file-streamer\resources\small.RAW">
        <cvParam cvRef="MS" accession="MS:1000768" value="" name="Thermo nativeID format" />
        <cvParam cvRef="MS" accession="MS:1000563" value="" name="Thermo RAW format" />
      </sourceFile>
    </sourceFileList>
  </fileDescription>
  <referenceableParamGroupList count="1">
    <referenceableParamGroup id="commonInstrumentParams">
      <cvParam cvRef="MS" accession="MS:1000448" value="" name="LTQ FT" />
      <cvParam cvRef="MS" accession="MS:1000529" value="SN06061F" name="instrument serial number" />
    </referenceableParamGroup>
  </referenceableParamGroupList>
  <sampleList count="1">
    <sample id="sample_1">
      <cvParam cvRef="MS" accession="MS:1000001" value="1" name="sample number" />
      <cvParam cvRef="NCIT" accession="NCIT:C41275" value="1a1" name="Vial" />
      <cvParam cvRef="NCIT" accession="NCIT:C43378" value="2" name="Row" />
      <cvParam cvRef="AFO" accession="AFQ:0000178" value="1" name="dilution factor" />
    </sample>
  </sampleList>
  <softwareList count="1">
    <software id="ThermoRawFileParser" version="1.2.3">
      <cvParam cvRef="MS" accession="MS:1000799" value="ThermoRawFileParser" name="custom unreleased software tool" />
    </software>
  </softwareList>
  <instrumentConfigurationList count="2">
    <instrumentConfiguration id="IC1">
      <referenceableParamGroupRef ref="commonInstrumentParams" />
      <componentList count="3">
        <source order="1">
          <cvParam cvRef="MS" accession="MS:1000073" value="" name="electrospray ionization" />
        </source>
        <analyzer order="2">
          <cvParam cvRef="MS" accession="MS:1000079" value="" name="fourier transform ion cyclotron resonance mass spectrometer" />
        </analyzer>
        <detector order="3">
          <cvParam cvRef="MS" accession="MS:1000624" value="" name="inductive detector" />
        </detector>
      </componentList>
    </instrumentConfiguration>
    <instrumentConfiguration id="IC2">
      <referenceableParamGroupRef ref="commonInstrumentParams" />
      <componentList count="3">
        <source order="1">
          <cvParam cvRef="MS" accession="MS:1000073" value="" name="electrospray ionization" />
        </source>
        <analyzer order="2">
          <cvParam cvRef="MS" accession="MS:1000264" value="" name="ion trap" />
        </analyzer>
        <detector order="3">
          <cvParam cvRef="MS" accession="MS:1000253" value="" name="electron multiplier" />
        </detector>
      </componentList>
    </instrumentConfiguration>
  </instrumentConfigurationList>
  <dataProcessingList count="1">
    <dataProcessing id="ThermoRawFileParserProcessing">
      <processingMethod order="0" softwareRef="ThermoRawFileParser">
        <cvParam cvRef="MS" accession="MS:1000544" value="" name="Conversion to mzML" />
      </processingMethod>
    </dataProcessing>
  </dataProcessingList>
  <run id="small" defaultInstrumentConfigurationRef="IC1" startTimeStamp="2005-07-20T14:44:22.377Z" defaultSourceFileRef="RAW1" sampleRef="sample_1"/></mzML>
"#;*/