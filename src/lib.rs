mod bindings;
pub mod mono;
pub mod mzml;
pub mod mzml_spectrum;
pub mod streamer;
pub mod prelude;

pub use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    //use anyhow::*;

    #[test]
    fn deserialize_mzml_metadata() {

        let mzml_header_str = r#"<?xml version="1.0" encoding="utf-8"?>
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
      <sourceFile id="RAW1" name="small" location="file:///./resources/small.RAW">
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
  <run id="small" defaultInstrumentConfigurationRef="IC1" startTimeStamp="2005-07-20T14:44:22.377Z" defaultSourceFileRef="RAW1" sampleRef="sample_1"/>
</mzML>
"#;
        let mzml_metadata = parse_mzml_metadata(mzml_header_str).unwrap();
        assert_eq!(mzml_metadata.run.default_source_file_ref, "RAW1");
        assert_eq!(mzml_metadata.software_list.software_entries.first().unwrap().version, "1.2.3");

        let first_sample_opt = mzml_metadata.sample_list.samples.first();
        assert!(first_sample_opt.is_some(), "can't find the first defined sample");

        let sample_number_param_opt = first_sample_opt.unwrap().cv_params.iter().find(|cvp| cvp.accession.as_str().eq("MS:1000001"));
        assert!(sample_number_param_opt.is_some());

        assert_eq!(sample_number_param_opt.unwrap().value.as_ref().unwrap(), "1");
    }

    const RAW_FILE_PARSER_PATH_STR: &'static str =
        if cfg!(debug_assertions) {
            "./target/debug/rawfileparser"
        } else {
            "./target/release/rawfileparser"
        };

    // WARNING: it is not possible to run multiple tests because rust unit tests are started in different threads
    #[test]
    fn get_spectra() {
        MONO_EMBEDDINATOR.lock().unwrap().configure(RAW_FILE_PARSER_PATH_STR).expect("e4k config failed");

        let streamer = RawFileStreamer::new("./resources/small.RAW").expect("streamer creation failed");

        let s1_data = streamer.get_spectrum_data(1).expect("get_spectrum failed");
        assert_eq!(s1_data.mz_list.len(), 1750, "inconsistency between expected and obtained number of m/z values");
        assert_eq!(s1_data.intensity_list.len(), 1750, "inconsistency between expected and obtained number of intensity values");

        let mut total_n_peaks = 0;
        for s_num in streamer.get_first_scan_number() ..= streamer.get_last_scan_number() {
            total_n_peaks += streamer.get_spectrum_data(s_num).expect("get_spectrum failed").mz_list.len()
        }

        assert_eq!(total_n_peaks, 47971, "inconsistency between expected and obtained total number of peaks");
    }

    #[test]
    fn multithreading_fails() {
        assert!(MONO_EMBEDDINATOR.lock().unwrap().check_availability().is_err());
     }
}
