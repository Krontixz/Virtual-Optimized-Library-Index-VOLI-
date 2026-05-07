use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Result};
use crate::volic::core::VolicClass;
use crate::volic::types::VolicType;

pub struct VolicConverter;

impl VolicConverter {
    pub fn convert_path(input_path: &str) -> Result<()> {
        let root = Path::new(input_path);
        let out_root = env::current_dir().unwrap().join("classes");

        if root.is_dir() {
            for entry in fs::read_dir(root)? {
                let entry = entry?;
                let path = entry.path();
                Self::process_item(&path, &out_root, root)?;
            }
        } else {
            Self::process_item(root, &out_root, root.parent().unwrap_or(Path::new(".")))?;
        }
        Ok(())
    }

    fn process_item(path: &Path, out_root: &PathBuf, base_input: &Path) -> Result<()> {
        let rel_path = path.strip_prefix(base_input).unwrap();
        let mut target_path = out_root.join(rel_path);

        if path.is_dir() {
            fs::create_dir_all(&target_path)?;
            for entry in fs::read_dir(path)? {
                Self::process_item(&entry?.path(), out_root, base_input)?;
            }
        } else {
            let mut file_data = Vec::new();
            fs::File::open(path)?.read_to_end(&mut file_data)?;

            let v_type = Self::detect_type(path);
            let volic = VolicClass::new(v_type, 1, file_data);
            
            target_path.set_extension("volic");
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let mut out_file = fs::File::create(target_path)?;
            volic.save(&mut out_file)?;
        }
        Ok(())
    }

    fn detect_type(path: &Path) -> VolicType {
        match path.extension().and_then(|s| s.to_str()) {
            Some("class") => VolicType::Jvm,
            Some("pyc") | Some("pyo") => VolicType::Python,
            Some("wasm") => VolicType::Wasm,
            Some("lua") | Some("luac") => VolicType::Lua,
            Some("beam") => VolicType::Beam,
            Some("exe") | Some("dll") => VolicType::DotNet,
            _ => VolicType::Unknown,
        }
    }
}
