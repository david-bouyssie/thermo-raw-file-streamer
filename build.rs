use std::env;
use std::fs;
use std::path::Path;
//use std::process::Command;

const RAW_FILE_PARSER_DIR: &str = "./resources/rawfileparser";

#[cfg(target_os = "windows")]
const MONO_INCLUDE_DIR: &str = r"C:\Program Files\Mono\include\mono-2.0\";

#[cfg(target_os = "linux")]
const MONO_INCLUDE_DIR: &str = "/usr/include/mono-2.0/";

/*
#[cfg(target_os = "windows")]
struct MSVCCompilationPaths {
    vs_path: String,
    msvc_path: String,
    msvc_version: String,
    ucrt_include_path: String,
    windows_sdk_include_path: String,
}

fn discover_msvc_compilation_paths() -> MSVCCompilationPaths {
    let vswhere_output = Command::new(r"C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe")
        .args(&["-latest", "-property", "installationPath"])
        .output()
        .expect("Failed to execute vswhere");

    let vswhere_output_str = String::from_utf8_lossy(&vswhere_output.stdout);
    let vs_path = vswhere_output_str.trim();
    let msvc_path = format!("{}\\VC\\Tools\\MSVC", vs_path);
    let msvc_version = find_latest_version_in_subdir(&msvc_path);

    let ucrt_include_path = format!("{}\\{}", msvc_path, &msvc_version);

    let windows_sdk_include_dir = r"C:\Program Files (x86)\Windows Kits\10\Include".to_string();
    let windows_sdk_version = find_latest_version_in_subdir(&windows_sdk_include_dir);
    let windows_sdk_include_path = format!("C:\\Program Files (x86)\\Windows Kits\\10\\Include\\{}", windows_sdk_version);

    MSVCCompilationPaths {
        vs_path: vs_path.to_string(),
        msvc_path,
        msvc_version,
        ucrt_include_path,
        windows_sdk_include_path,
    }
}

fn find_latest_version_in_subdir(directory: &String) -> String {
    // Enumerate subdirectories in the MSVC path
    let entries = fs::read_dir(directory).expect("Failed to read directory content");
    let mut versions = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(version) = entry.file_name().into_string() {
                versions.push(version);
            }
        }
    }

    // Sort versions and take the first one
    versions.sort();
    versions.last().expect("No sub-directory found").to_string()
}*/

macro_rules! warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = out_dir.split("build").next().unwrap().to_string();

    // Copy C# libraries in the target directory (skip if already there)
    fs_extra::dir::copy(
        RAW_FILE_PARSER_DIR,
        target_dir.clone(),
        &fs_extra::dir::CopyOptions::new().skip_exist(true)
    ).expect("Failed to copy directory");

    // Conditionally add the include path only on Windows
    // Note fur future: linking on Windows currently relies on the shared library mono-2.0-sgen.dll
    // but we could consider static linking in the Future by using libmono-static-sgen.lib
    if cfg!(target_os = "windows") {

        // Unfortunately this doesn't work yet
        /*warn!("Compiling glue code using the following MSVC toolchain");

        let msvc_paths = discover_msvc_compilation_paths();
        warn!("- VS Path: {}", msvc_paths.vs_path);
        warn!("- MSVC Path: {}", msvc_paths.msvc_path);
        warn!("- MSVC Version: {}", msvc_paths.msvc_version);
        warn!("- UCRT Include Path: {}", msvc_paths.ucrt_include_path);
        warn!("- Windows SDK Include Path: {}", msvc_paths.windows_sdk_include_path);

        // Compile the Embeddinator-4000 generated glue code
        // (code taken from https://github.com/david-bouyssie/ThermoRawFileParserBindings/tree/master/c)
        cc::Build::new()
            .file(r".\resources\bindings\ThermoRawFileParser.c")
            .include(msvc_paths.ucrt_include_path)
            .include(msvc_paths.windows_sdk_include_path)
            .include(MONO_INCLUDE_DIR)
            .shared_flag(true) // Specify that the library should be compiled as a shared library
            .flag("/LD") // Add the /LD flag to force cl.exe to create a dynamic library
            .out_dir(target_dir.clone())
            .compile("ThermoRawFileParser");*/

        // Copy Embeddinator glue code compiled as Windows shared library to the target directory
        let thermo_glue_dll_path = Path::new(&target_dir).join("ThermoRawFileParser.dll");
        if thermo_glue_dll_path.exists() == false {
            fs::copy(r".\resources\lib\ThermoRawFileParser.dll", thermo_glue_dll_path.to_owned()).unwrap();
        }
        let thermo_glue_lib_path = Path::new(&target_dir).join("ThermoRawFileParser.lib");
        if thermo_glue_lib_path.exists() == false {
            fs::copy(r".\resources\lib\ThermoRawFileParser.lib", thermo_glue_lib_path.to_owned()).unwrap();
        }

        // Copy Mono shared library to the target directory
        // FIXME: we should use something more configurable (maybe MONO_HOME env var?)
        let mono_dll_path = Path::new(&target_dir).join("mono-2.0-sgen.dll");
        if mono_dll_path.exists() == false {
            fs::copy(r"C:\Program Files\Mono\bin\mono-2.0-sgen.dll", mono_dll_path.to_owned()).unwrap();
        }
        let mono_lib_path = Path::new(&target_dir).join("monosgen-2.0.lib");
        if mono_lib_path.exists() == false {
            fs::copy(r"C:\Program Files\Mono\lib\mono-2.0-sgen.lib", mono_lib_path.to_owned()).unwrap();
        }
    } else if cfg!(target_os = "linux") {
        // Copy Embeddinator glue code compiled as Linux shared library to the target directory
        // Note: this still requires to execute a line like this before executing the main program
        // export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(dirname $(readlink -f $0))/
        let thermo_glue_so_path = Path::new(&target_dir).join("libThermoRawFileParser.so");
        if thermo_glue_so_path.exists() == false {
            fs::copy(r"./resources/lib/libThermoRawFileParser.so", thermo_glue_so_path.to_owned()).unwrap();
        }
    } else {
        panic!("Your OS is not yet supported!")
    }

    let bindings_path = "./src/bindings.rs";
    if !Path::new(&bindings_path).exists() {
        warn!("Generating new binding code for ThermoRawFileParser.h");

        let bindings = bindgen::Builder::default()
            .header("./resources/bindings/ThermoRawFileParser.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .clang_arg(format!("-I{}", MONO_INCLUDE_DIR));

        let bindings = bindings.generate().expect("Unable to generate bindings");

        bindings
            .write_to_file(bindings_path)
            .expect("Couldn't write bindings!");
    }

    // --- Link native shared library (E4K glue code) --- //
    println!("cargo:rustc-link-search=native={}",target_dir);
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=ThermoRawFileParser");
    }
}