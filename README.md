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

 (using [pythonnet](https://github.com/pythonnet/pythonnet)). Since Microsoft introduced .NET Standard it is possible to load DLLs compiled with this framework to be loaded on non-Windows systems (such as Mac OS and Linux). However, systems other than Windows may require additional setup steps in order for fisher_py to work.
If you have trouble problems installing fisher_py it is probably because of pythonnet not being able to compile. To resolve this the usualy path is to install mono (https://www.mono-project.com/). There are several guides online to do this but one that was tested can be found [here](https://linuxize.com/post/how-to-install-mono-on-ubuntu-20-04/).

## Examples
The following example demonstrates how to extract and plot data from a raw-file:
```python
import matplotlib.pyplot as plt
from fisher_py import RawFile
from fisher_py.data.business import TraceType
raw_file = RawFile('my_file.raw')

target_mass = 848.36862
mass_tolerance_ppm = 10
rt, i = raw_file.get_chromatogram(target_mass, mass_tolerance_ppm, TraceType.MassRange)
mz, i2, charges, real_rt = raw_file.get_scan_ms1(1)
print(real_rt)

plt.figure()
plt.plot(rt, i)

plt.figure()
plt.plot(mz, i2)

plt.show()
```

This example may be fine for some use-cases but the RawFile class only provides limited access to all the functionalities and can serve as an example how to use the module wihtin a project.
For an example that uses more of the modules capabilites have a look at [raw_file_reader_examle.py](https://github.com/ethz-institute-of-microbiology/fisher_py/blob/main/examples/raw_file_reader_example.py).

## License and copyright
fisher_py (Copyright 2021 ethz-institute-of-microbiology) is licensed under the  MIT license.

### Third-party licenses and copyright

RawFileReader reading tool. Copyright © 2016 by Thermo Fisher Scientific, Inc. All rights reserved. See [RawFileReaderLicense.md](https://github.com/ethz-institute-of-microbiology/fisher_py/blob/main/RawFileReaderLicense.md) for licensing information. 
Note: anyone recieving RawFileReader as part of a larger software distribution (in the current context, as part of fisher_py) is considered an "end user" under 
section 3.3 of the RawFileReader License, and is not granted rights to redistribute RawFileReader.

### Remarks
Some parts of the code were ported from a previous Scala project:
https://github.com/mzdb/mzdb4s/tree/master/io-thermo

