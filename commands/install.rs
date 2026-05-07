use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::Result;

pub struct VoliInstaller {
    pub home_dir: PathBuf,
}

impl VoliInstaller {
    pub fn new() -> Self {
        let mut path = env::current_dir().unwrap();
        path.push(".voli");
        Self { home_dir: path }
    }

    pub fn setup_environment(&self) -> Result<()> {
        if !self.home_dir.exists() {
            fs::create_dir_all(&self.home_dir)?;
        }

        let bin_path = self.home_dir.join("bin");
        let lib_path = self.home_dir.join("lib");
        let cache_path = self.home_dir.join("cache");
        let classes_path = env::current_dir().unwrap().join("classes");

        fs::create_dir_all(bin_path)?;
        fs::create_dir_all(lib_path)?;
        fs::create_dir_all(cache_path)?;
        fs::create_dir_all(classes_path)?;

        Ok(())
    }

    pub fn register_extension(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            Command::new("assoc").arg(".volic=VolicFile").output()?;
            Command::new("assoc").arg(".volis=VolisStack").output()?;
            Command::new("ftype").arg("VolicFile=voli.exe %1").output()?;
            Command::new("ftype").arg("VolisStack=voli.exe %1").output()?;
        }
        Ok(())
    }
}
