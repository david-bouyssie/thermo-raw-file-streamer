
use anyhow::*;
use std::ffi::c_char;
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;
use path_absolutize::Absolutize;

use crate::bindings::*;

lazy_static! {
    pub static ref MONO_EMBEDDINATOR: Mutex<MonoEmbeddinator> = Mutex::new(MonoEmbeddinator::new());
    //pub mut static ref MONO_EMBEDDINATOR: MonoEmbeddinator = MonoEmbeddinator::new();
}

#[derive(Clone, Debug)]
pub struct MonoEmbeddinator {
    configured: bool,
    disposed: bool,
}

/*impl Drop for MonoEmbeddinator {
    fn drop(&mut self) {
        if !self.disposed {
            eprintln!("Disposing Mono Embeddinator...");
            let _ = self.dispose(); // shall we panic on error?
        }
    }
}*/

impl MonoEmbeddinator {
    fn new() -> Self {
        Self {
            configured: false,
            disposed: false,
        }
    }

    /*pub fn is_configured(&self) -> bool {
        self.configured
    }

    pub fn is_disposed(&self) -> bool {
        self.disposed
    }*/

    pub fn check_availability(&self) -> Result<()> {
        if !self.configured {
            bail!("Mono Embeddinator is not configured");
        }
        if self.disposed {
            bail!("Mono Embeddinator is disposed");
        }

        Ok(())
    }

    pub fn configure(&mut self, raw_file_parser_directory: &str) -> Result<()> {
        if self.disposed {
            bail!("can't configure Mono Embeddinator since it has been disposed");
        }

        if self.configured {
            bail!("Mono Embeddinator can't be configured twice!");
        }

        // Define Mono runtime location
        /*let runtime_assembly_path = if cfg!(windows) {
            let raw_file_parser_abs_dir = Path::new(&self.raw_file_parser_directory)
                .canonicalize()?
                .to_str()
                .unwrap()
                .replace('\\', "/") + "/";
            raw_file_parser_abs_dir
        } else {
            get_mono_runtime_assembly_path_on_linux() + "/"
        };

        let mono_directory_path = Path::new(&runtime_assembly_path).join("mono");
        if !mono_directory_path.is_dir() {
            bail!("can't find Mono runtime at '{}'", runtime_assembly_path)
        }

        unsafe {
            mono_embeddinator_set_assembly_path(runtime_assembly_path.as_ptr() as *const c_char);
        }

        let runtime_assembly_path_str = runtime_assembly_path.as_str();
        if !Path::new(runtime_assembly_path_str).join("mono").is_dir() {
            bail!( "can't find Mono runtime at '{}'", runtime_assembly_path);
        }
        */

        // Define Mono runtime location (note: missing ending slash prevents Mono to load properly)
        let runtime_assembly_path = if cfg!(windows) {

            let raw_file_parser_abs_dir = Path::new(&raw_file_parser_directory).absolutize()?
                .to_str()
                .unwrap()
                .replace('\\', "/") + "/";

            raw_file_parser_abs_dir
        } else {
            MonoEmbeddinator::_get_mono_runtime_assembly_path_on_linux() + "/"
        };

        let mono_directory_path = Path::new(&runtime_assembly_path).join("mono");
        if !mono_directory_path.is_dir() {
            bail!("can't find Mono runtime at '{}'", runtime_assembly_path)
        }

        unsafe {
            mono_embeddinator_set_assembly_path(runtime_assembly_path.as_ptr() as *const c_char);
        }

        let runtime_assembly_path_str = runtime_assembly_path.as_str();
        if !Path::new(runtime_assembly_path_str).join("mono").is_dir() {
            bail!( "can't find Mono runtime at '{}'", runtime_assembly_path);
        }

        // Set Mono runtime location
        unsafe { mono_embeddinator_set_runtime_assembly_path(runtime_assembly_path_str.as_ptr() as *const c_char) }

        self.configured = true;

        Ok(())
    }

    pub fn dispose(&mut self) -> Result<()> {
        if !self.configured {
            bail!("can't dispose Mono Embeddinator since it has not been previously configured");
        }

        if self.disposed {
            bail!("Mono Embeddinator has been already disposed");
        }

        unsafe {
            if mono_embeddinator_get_context().is_null() || mono_domain_get().is_null()  {
                bail!("Mono JIT runtime has been initiated yet.");
            }

            // There are alternatives but it's not clear to me what is the best one
            // For now let's take the same approache than Embeddinator-4000
            //mono_assemblies_cleanup();
            //mono_runtime_cleanup(mono_domain_get());
            //mono_runtime_quit();
            mono_jit_cleanup(mono_domain_get());

            // I think mono_embeddinator_destroy predicate is wrong (ctx->domain != 0)
            /*if mono_embeddinator_destroy(mono_embeddinator_get_context()) == 0 {
                bail!("can't call mono_embeddinator_destroy (undefined Mono context or domain)");
            }*/

            // Reset assembly and runtime assembly paths
            //mono_embeddinator_set_assembly_path(std::ptr::null());
            //mono_embeddinator_set_runtime_assembly_path(std::ptr::null());

            self.disposed = true;

            Ok(())
        }
    }

    // And this function only gets compiled if the target OS is *not* linux
    fn _get_mono_runtime_assembly_path_on_linux() -> String {
        // MONO_PATH should refer to a fully defined and absolute path to MONO (like "/usr/lib/mono/4.0")
        match std::env::var("MONO_PATH") {
            Result::Ok(mono_path) => {
                Some(mono_path.split("mono").next().unwrap_or(&mono_path)).unwrap_or(&mono_path).to_string()
            }
            Err(_) => "/usr/lib/".to_string(),
        }
    }
}

/*pub fn configure_mono_embeddinator(raw_file_parser_directory: &str) -> Result<()> {
    unsafe {
        if !mono_embeddinator_get_context().is_null() {
            bail!("Mono Embeddinator can't be configured twice!")
        }
    }

    unsafe { println!("mono_embeddinator_get_context={}", mono_embeddinator_get_context().is_null()) ; }

    // Define Mono runtime location (note: missing ending slash prevents Mono to load properly)
    let runtime_assembly_path = if cfg!(windows) {

        let raw_file_parser_abs_dir = Path::new(raw_file_parser_directory).absolutize()?
            .to_str()
            .unwrap()
            .replace('\\', "/") + "/";

        raw_file_parser_abs_dir
    } else {
        get_mono_runtime_assembly_path_on_linux() + "/"
    };

    let mono_directory_path = Path::new(&runtime_assembly_path).join("mono");
    if !mono_directory_path.is_dir() {
        bail!("can't find Mono runtime at '{}'", runtime_assembly_path)
    }

    unsafe {
        mono_embeddinator_set_assembly_path(runtime_assembly_path.as_ptr() as *const c_char);
    }

    let runtime_assembly_path_str = runtime_assembly_path.as_str();
    if !Path::new(runtime_assembly_path_str).join("mono").is_dir() {
        bail!( "can't find Mono runtime at '{}'", runtime_assembly_path);
    }

    // Set Mono runtime location
    unsafe { mono_embeddinator_set_runtime_assembly_path(runtime_assembly_path_str.as_ptr() as *const c_char) }

    // Initialize Mono Embeddinator
    /*unsafe {
        use std::alloc::{alloc, Layout, dealloc};

        let size = std::mem::size_of::<mono_embeddinator_context_t>();
        let align = std::mem::align_of::<mono_embeddinator_context_t>();
        let layout = Layout::from_size_align(size, align).expect("Invalid layout");

        // Allocate memory
        let context_ptr = unsafe { alloc(layout) as *mut mono_embeddinator_context_t };

        // Ensure the allocation was successful
        if context_ptr.is_null() {
            panic!("Failed to allocate memory.");
        }

        let domain_str = std::ffi::CString::new("mono_embeddinator_binding")?.as_ptr();
        mono_embeddinator_init(context_ptr, domain_str);
    }*/

    unsafe { println!("mono_embeddinator_get_context={}", mono_embeddinator_get_context().is_null()) ; }

    Ok(())

}

// Becareful, disposing the Mono JIT may prevent future use of code relying on Embeddinator-4000
pub fn dispose_mono_embeddinator() -> Result<()> {
    unsafe {
        if mono_embeddinator_get_context().is_null() || mono_domain_get().is_null()  {
            bail!("Mono JIT runtime has been initiated yet.");
        }

        // There are alternatives but it's not clear to me what is the best one
        // For now let's take the same approache than Embeddinator-4000
        //mono_assemblies_cleanup();
        //mono_runtime_cleanup(mono_domain_get());
        //mono_runtime_quit();
        mono_jit_cleanup(mono_domain_get());

        /*let null_ptr: *mut mono_embeddinator_context_t = std::ptr::null_mut();
        mono_embeddinator_set_context(null_ptr);*/

        /*let domain_str = std::ffi::CString::new("mono_embeddinator_binding")?.as_ptr();
        mono_embeddinator_init(mono_embeddinator_get_context(), domain_str);

        unsafe { println!("mono_embeddinator_get_context2={}", mono_embeddinator_get_context().is_null()) ; }*/

        // Rset assembly and runtime assembly paths
        mono_embeddinator_set_assembly_path(std::ptr::null());
        mono_embeddinator_set_runtime_assembly_path(std::ptr::null());


        // I think mono_embeddinator_destroy predicate is wrong (ctx->domain != 0)
        /*if mono_embeddinator_destroy(mono_embeddinator_get_context()) == 0 {
            bail!("can't call mono_embeddinator_destroy (undefined Mono context or domain)");
        }*/

        Ok(())
    }
}*/

