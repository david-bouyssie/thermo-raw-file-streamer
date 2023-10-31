#![allow(unused)]

use std::borrow::BorrowMut;
use anyhow::*;
use std::ffi::{CStr, CString};
use std::path::Path;
use path_absolutize::Absolutize;

use crate::bindings::*;
use crate::{mzml, mzml_spectrum};
use crate::mono::MONO_EMBEDDINATOR;
use crate::mzml::{MzMLMetaData};
use crate::mzml_spectrum::{MzMLSpectrum, MzMLSpectrumMetaData, SpectrumData};

pub fn get_thermo_raw_file_parser_version() -> Result<String> { // Result<String>
    MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

    unsafe {
        Ok(cstr_to_string(ThermoRawFileParser_MainClass_get_Version()))
    }
}

#[inline(always)]
unsafe fn cstr_to_string(cstr: *const std::os::raw::c_char) -> String {
    let c_string = CStr::from_ptr(cstr);
    c_string.to_string_lossy().into_owned()
}

#[derive(Clone, Debug)]
pub struct RawFileStreamer {
    raw_file_path: String,
    first_scan_number: u32,
    last_scan_number: u32,
    meta_data: MzMLMetaData,
    raw_file_wrapper_ptr: *mut ThermoRawFileParser_RawFileWrapper,
    mzml_writer_ptr: *mut ThermoRawFileParser_Writer_MzMlSpectrumWriter,
    spectrum_wrapper_ptr: *mut ThermoRawFileParser_Writer_SpectrumWrapper
}

impl Drop for RawFileStreamer {
    fn drop(&mut self) {
        self.dispose()
    }
}

impl RawFileStreamer {

    pub fn dispose(&self) {
        if MONO_EMBEDDINATOR.lock().unwrap().check_availability().is_ok() {
            unsafe {
                // TODO: double check if this is enough
                ThermoRawFileParser_RawFileWrapper_Dispose(self.raw_file_wrapper_ptr)
            }
        }
    }

    pub fn new(raw_file_path: &str) -> Result<RawFileStreamer> {
        MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

        unsafe {
            let abs_raw_file_path: String = Path::new(raw_file_path).absolutize()?.display().to_string();
            let raw_file_path_cstr = CString::new(abs_raw_file_path)?;

            let parse_input = ThermoRawFileParser_ParseInput_new_1(
                raw_file_path_cstr.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                ThermoRawFileParser_OutputFormat_ThermoRawFileParser_OutputFormat_MzML,
            );

            ThermoRawFileParser_ParseInput_set_UseInMemoryWriter(parse_input, true);

            let raw_file_wrapper = ThermoRawFileParser_RawFileParser_InitRawFile(parse_input);

            let first_scan_number = ThermoRawFileParser_RawFileWrapper_get_FirstScanNumber(raw_file_wrapper);
            let last_scan_number = ThermoRawFileParser_RawFileWrapper_get_LastScanNumber(raw_file_wrapper);

            let mzml_writer = ThermoRawFileParser_Writer_MzMlSpectrumWriter_new(raw_file_wrapper);

            ThermoRawFileParser_Writer_MzMlSpectrumWriter_CreateXmlWriter(mzml_writer);

            ThermoRawFileParser_Writer_MzMlSpectrumWriter_WriteHeader(
                mzml_writer,
                first_scan_number,
                last_scan_number,
            );

            let meta_data_as_xml_string = cstr_to_string(
                ThermoRawFileParser_Writer_MzMlSpectrumWriter_GetInMemoryStreamAsString(mzml_writer)
            ) + "/></mzML>";

            //println!("XML header:\n{}", meta_data_as_xml_string);

            let meta_data = mzml::parse_mzml_metadata(&meta_data_as_xml_string)?;

            let spectrum_wrapper_ptr = ThermoRawFileParser_Writer_MzMlSpectrumWriter_getSpectrumWrapper(mzml_writer);

            Ok(RawFileStreamer {
                raw_file_path: raw_file_path.to_string(),
                first_scan_number: first_scan_number as u32,
                last_scan_number: last_scan_number as u32,
                meta_data: meta_data,
                raw_file_wrapper_ptr: raw_file_wrapper,
                mzml_writer_ptr: mzml_writer,
                spectrum_wrapper_ptr,
            })
        }
    }

    pub fn get_raw_file_path(&self) -> &str {
        &self.raw_file_path
    }

    pub fn get_first_scan_number(&self) -> u32 {
        self.first_scan_number
    }

    pub fn get_last_scan_number(&self) -> u32 {
        self.last_scan_number
    }

    pub fn process_spectra_in_parallel<F>(&self, mut on_each_spectrum: F, queue_size: usize) -> Result<()>
    where
        F: FnMut(Result<MzMLSpectrum>) -> Result<()> + Send + Sync,
     {

        let (sender_queue, receiver_queue) = std::sync::mpsc::sync_channel(queue_size);

        let x = std::thread::scope(|thread_scope| {
            let processing_thread = thread_scope.spawn(move || {
                let mut sn = 0;
                for spec_res in receiver_queue.iter() {
                    sn += 1;
                    //println!("{}", sn);

                    on_each_spectrum(spec_res);

                    /*if sn % 1000 == 0 {
                        println!("Processed {} spectra...", sn);
                    }*/
                }

                println!("All spectra have been processed!");

                ()
            });

            self._enqueue_all_spectra(sender_queue)?;

            processing_thread.join().or_else(|e| {
                let error_message = if let Some(err) = e.downcast_ref::<&str>() {
                    anyhow!(err.to_string())
                } else if let Some(err) = e.downcast_ref::<String>() {
                    anyhow!(err.to_string())
                } else {
                    anyhow!("Unknown error occurred in the processing thread")
                };
                Err(error_message)
            })
        })?;

        Ok(())
    }

    fn _enqueue_all_spectra(&self, queue: std::sync::mpsc::SyncSender<Result<MzMLSpectrum>>) -> Result<()> {

        MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

        for spec_num in 1 ..= self.get_last_scan_number() {
            println!("spec_num={}", spec_num);

            let spectrum_res = self._get_spectrum(spec_num, true, true).map(|(metadata_opt,data_opt)| {
                MzMLSpectrum::new(metadata_opt.unwrap(), data_opt.unwrap())
            });

            queue.send(spectrum_res);
        }

        // Dropping the queue to signal that no more items will be sent
        drop(queue);

        Ok(())
    }

    pub fn get_spectrum(&self, number: u32) -> Result<MzMLSpectrum> {
        MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

        let (metadata_opt,data_opt) = self._get_spectrum(number, true, true)?;
        let spectrum = MzMLSpectrum::new(metadata_opt.unwrap(), data_opt.unwrap());

        Ok(spectrum)
    }

    pub fn get_spectrum_metadadata(&self, number: u32) -> Result<MzMLSpectrumMetaData> {
        MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

        self._get_spectrum(number, false, true).map(|tuple| tuple.0.unwrap())
    }

    pub fn get_spectrum_data(&self, number: u32) -> Result<SpectrumData> {
        MONO_EMBEDDINATOR.lock().unwrap().check_availability()?;

        self._get_spectrum(number, true, false).map(|tuple| tuple.1.unwrap())
    }

    // TODO: we may want to use something like realloc to maintain a single buffer instead of allocating memory every time
    fn _get_spectrum(&self, number: u32, load_data: bool, load_metadata: bool) -> Result<(Option<MzMLSpectrumMetaData>,Option<SpectrumData>)> {

        if number < self.first_scan_number {
            bail!("requested spectrum number ({}) is lower than first scan number ({})", number, self.first_scan_number);
        }
        if number > self.last_scan_number {
            bail!("requested spectrum number ({}) is higher than last scan number ({})", number, self.last_scan_number);
        }

        unsafe {
            let scan_number = number as i32;
            ThermoRawFileParser_Writer_MzMlSpectrumWriter_ResetWriter(self.mzml_writer_ptr, false);
            ThermoRawFileParser_Writer_MzMlSpectrumWriter_WriteSpectrumNoReturn(self.mzml_writer_ptr, scan_number , scan_number, false);

            let metadata_opt = if load_metadata {
                Some(self._retrieve_spectrum_meta_data()?)
            } else{
                None
            };

            let data_opt = if load_data {
                Some(self._retrieve_spectrum_data()?)
            } else {
                None
            };

            Ok((metadata_opt, data_opt) )
        }
    }

    unsafe fn _retrieve_spectrum_meta_data(&self) -> Result<MzMLSpectrumMetaData> {

        let xml_chunk_len = ThermoRawFileParser_Writer_MzMlSpectrumWriter_FlushWriterThenGetXmlStreamLength(self.mzml_writer_ptr);

        // Allocate memory
        let layout = std::alloc::Layout::from_size_align(xml_chunk_len as usize, std::mem::align_of::<u8>()).unwrap();
        let xml_chunk_ptr = std::alloc::alloc(layout) as *mut u8;
        let xml_chunk_ptr_address = xml_chunk_ptr as usize as i64;

        ThermoRawFileParser_Writer_MzMlSpectrumWriter_CopyXmlStreamToPointers(self.mzml_writer_ptr, xml_chunk_ptr_address);

        let xml_chunk_bytes= std::slice::from_raw_parts(xml_chunk_ptr, xml_chunk_len as usize);
        let xml_chunk = std::str::from_utf8(xml_chunk_bytes)?;
        //let xml_chunk = String::from_utf8(xml_chunk_bytes.to_vec())?;
        //println!("xml_chunk={}", xml_chunk);

        let mzml_spectrum_metada_data = mzml_spectrum::parse_mzml_spectrum_metadata(xml_chunk)?;

        // Deallocate memory for XML chunk
        std::alloc::dealloc(xml_chunk_ptr as *mut u8, layout);

        Ok(mzml_spectrum_metada_data)
    }

    unsafe fn _retrieve_spectrum_data(&self) -> Result<SpectrumData> {

        let peaks_count = ThermoRawFileParser_Writer_SpectrumWrapper_getPeaksCount(self.spectrum_wrapper_ptr) as usize;

        if peaks_count == 0  {
            Ok(SpectrumData { mz_list: vec![], intensity_list: vec![] })
        } else {

            // Calculate the total size in bytes
            let total_size = peaks_count * std::mem::size_of::<f64>();

            // Allocate memory for spectrum data
            let layout = std::alloc::Layout::from_size_align(total_size, std::mem::align_of::<f64>()).unwrap();

            let mz_ptr = std::alloc::alloc(layout) as *mut f64;
            let mz_ptr_address = mz_ptr as usize as i64;

            let intensity_ptr = std::alloc::alloc(layout) as *mut f64;
            let intensity_ptr_address = intensity_ptr as usize as i64;

            ThermoRawFileParser_Writer_SpectrumWrapper_CopyDataToPointers(self.spectrum_wrapper_ptr, mz_ptr_address, intensity_ptr_address);

            let mz_values: &[f64] = std::slice::from_raw_parts(mz_ptr as *const f64, peaks_count);
            let intensity_values: &[f64] = std::slice::from_raw_parts(intensity_ptr as *const f64, peaks_count);

            let spec_data = SpectrumData { mz_list: mz_values.to_vec(), intensity_list: intensity_values.to_vec() };

            // Deallocate memory for spectrum data
            std::alloc::dealloc(mz_ptr as *mut u8, layout);
            std::alloc::dealloc(intensity_ptr as *mut u8, layout);

            Ok(spec_data)
        }
    }
}

/*
fn alloc_bytes(n_bytes: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(xml_chunk_len as usize, std::mem::align_of::<u8>()).unwrap();
            let xml_chunk_ptr = std::alloc::alloc(layout) as *mut u8;
}*/

// Add functions missing in the automatically generated code (needs to be investigated)
#[link(name = "ThermoRawFileParser")] extern "C" {
    fn ThermoRawFileParser_Writer_MzMlSpectrumWriter_FlushWriterThenGetXmlStreamLength(
        object: *mut ThermoRawFileParser_Writer_MzMlSpectrumWriter
    ) -> i32;
}
#[link(name = "ThermoRawFileParser")] extern "C" {
    fn ThermoRawFileParser_Writer_MzMlSpectrumWriter_CopyXmlStreamToPointers(
        object: *mut ThermoRawFileParser_Writer_SpectrumWrapper,
        ptr_address: i64,
    );
}