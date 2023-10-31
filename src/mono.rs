
use anyhow::*;
use std::ffi::{c_char, CString};
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
    init_thread_id: u64,
    assembly_path: Option<CString>,
    runtime_assembly_path: Option<CString>
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
            init_thread_id: 0,
            assembly_path: None,
            runtime_assembly_path: None
        }
    }

    pub fn get_current_thread_id_as_u64() -> Result<u64> {
        let raw_thread_id = format!("{:?}", std::thread::current().id());
        //dbg!("raw_thread_id={} {}", raw_thread_id.clone(), std::thread::current().id());
        let parsed_thread_id = raw_thread_id.trim_start_matches("ThreadId(")
            .trim_end_matches(")")
            .parse::<u64>()?;

        Ok(parsed_thread_id)
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }

    /*pub fn is_disposed(&self) -> bool {
        self.disposed
    }*/

    pub fn check_availability(&self) -> Result<()> {
        if !self.configured {
            bail!("Mono Embeddinator is not configured");
        }
        if self.disposed {
            bail!("Mono Embeddinator is disposed");
        }

        let cur_thread_id = MonoEmbeddinator::get_current_thread_id_as_u64()?;
        if self.init_thread_id != cur_thread_id {
            bail!("forbidden operation: Mono Embeddinator was initiated in thread '{}' but is now used from thread '{}'", self.init_thread_id, cur_thread_id);
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

        // Define Mono runtime location (note: missing ending slash prevents Mono to load properly)
        let raw_file_parser_abs_dir = Path::new(&raw_file_parser_directory).absolutize()?
            .to_string_lossy()
            .to_string()
            .replace('\\', "/") + "/";

        self.assembly_path = Some(CString::new(raw_file_parser_abs_dir.clone())?);

        unsafe {
            mono_embeddinator_set_assembly_path(self.assembly_path.as_ref().unwrap().as_ptr() as *const c_char);
        }

        let runtime_assembly_path = if cfg!(windows) {raw_file_parser_abs_dir}
        else {
            MonoEmbeddinator::_get_mono_runtime_assembly_path_on_linux() + "/"
        };

        let mono_directory_path = Path::new(&runtime_assembly_path).join("mono");
        if !mono_directory_path.is_dir() {
            bail!("can't find Mono runtime at '{:?}'", mono_directory_path)
        }

        self.runtime_assembly_path = Some(CString::new(runtime_assembly_path)?);

        // Set Mono runtime location
        unsafe { mono_embeddinator_set_runtime_assembly_path(self.runtime_assembly_path.as_ref().unwrap().as_ptr() as *const c_char) }

        self.configured = true;

        self.init_thread_id = MonoEmbeddinator::get_current_thread_id_as_u64()?;

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

            let mono_domain = mono_domain_get();
            mono_jit_cleanup(mono_domain);
            //mono_domain_free(mono_domain, 1);
            //mono_domain_unload(mono_domain);

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

