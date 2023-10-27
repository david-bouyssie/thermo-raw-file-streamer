# thermo-raw-file-streamer

This *experimental* library allows *direct* access to RAW files acquired on Thermo Fisher Scientific instruments.
This library is a wrapper of a [Compomics ThermoRawFileParser fork](https://github.com/david-bouyssie/ThermoRawFileParser) that streams the raw file content as mzML chunks, that can then be decoded from the caller side.
Similarly to [fisher_py](https://raw.githubusercontent.com/ethz-institute-of-microbiology/fisher_py), it allows to access the raw data prior additional file conversion, avoiding thus disk IO operations.

## How does it work?

The module relies on the ThermoRawFileParser and Thermo RawFileReader DLLs (C# libraries) to be loaded at runtime.

This C# interop is provided by the Mono embedding API and the C# glue code has been generated thanks to a fork of the Embeddinator-4000 tool:
* https://www.mono-project.com/docs/advanced/embedding/
* https://github.com/david-bouyssie/Embeddinator-4000

The C# code has been modified for optimal C# <-> foreign code (presently Rust) interoperability.
The mzML meta-data and the spectra vectors are memory-allocated from the caller side, which has to reclaim first
the number of bytes that needs to be allocated. So the caller has the responsibility of memory allocation/deallocation.
The memory address and arrays length are sent to the C# code, and will be used to store the meta-data as XML UTF-8 strings and the (mz, intensity) data as numeric arrays.
Hence, vectors of spectra values (mz, intensity) are treated in a way that avoids back and forth Base64 serialization round-trips, providing better performance.

## System Requirements
This library currently works only on Linux and Windows, and requires Mono to be installed:
* on Linux -> install mono-complete
* on Windows -> choco install mono or perform manual installation [Mono for Windows](https://www.mono-project.com/download/stable/#download-win)



### Remarks
Some parts of the code were ported from a previous Scala project:
https://github.com/mzdb/mzdb4s/tree/master/io-thermo

