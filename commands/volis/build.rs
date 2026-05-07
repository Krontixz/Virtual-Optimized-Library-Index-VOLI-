use std::fs;
use std::io::{Write, Result};
use std::path::Path;
use crate::volis::stack::{VolisStack, VolisEntry};

pub struct VolisBuilder;

impl VolisBuilder {
    pub fn build_stack(input_folder: &str, output_name: &str) -> Result<()> {
        let mut stack = VolisStack::new();
        let mut combined_data = Vec::new();
        let mut current_offset = 0u64;

        let entries = fs::read_dir(input_folder)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("volic") {
                let mut data = fs::read(&path)?;
                let size = data.len() as u64;
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();

                stack.add_entry(name, current_offset, size);
                combined_data.append(&mut data);
                current_offset += size;
            }
        }

        let mut out_file = fs::File::create(format!("{}.volis", output_name))?;
        out_file.write_all(b"VOLIS")?;
        out_file.write_all(&(stack.entries.len() as u32).to_be_bytes())?;
        
        for entry in &stack.entries {
            let name_bytes = entry.name.as_bytes();
            out_file.write_all(&(name_bytes.len() as u16).to_be_bytes())?;
            out_file.write_all(name_bytes)?;
            out_file.write_all(&entry.offset.to_be_bytes())?;
            out_file.write_all(&entry.size.to_be_bytes())?;
        }

        out_file.write_all(&combined_data)?;
        Ok(())
    }
}
