mod bindings;
pub mod mono;
pub mod mzml;
pub mod mzml_spectrum;
pub mod streamer;
pub mod prelude;

pub use prelude::*;

use anyhow::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn configure_embeddinator() {
        MONO_EMBEDDINATOR.lock().unwrap().configure("./target/debug/rawfileparser").expect("e4k config failed");
    }

    #[test]
    fn get_spectra() {

        let streamer = RawFileStreamer::new("./resources/small.RAW").expect("streamer creation failed");

        let (s1_meta,s1_data) = streamer.get_spectrum(1).expect("get_spectrum failed");
        assert_eq!(s1_data.mz_list.len(), 1750, "inconsistency between expected and obtained number of m/z values");
        assert_eq!(s1_data.intensity_list.len(), 1750, "inconsistency between expected and obtained number of intensity values");

        let mut total_n_peaks = 0;
        for s_num in streamer.get_first_scan_number() ..= streamer.get_last_scan_number() {
            total_n_peaks += streamer.get_spectrum(s_num).expect("get_spectrum failed").1.mz_list.len()
        }

        assert_eq!(total_n_peaks, 47971, "inconsistency between expected and obtained total number of peaks");
    }
}
