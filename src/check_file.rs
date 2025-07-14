use crate::error::PrettystrictError;
use std::fs;
use std::io::Read;

#[allow(dead_code)]
fn check(file: Option<String>, watch: bool) -> Result<(), PrettystrictError> {
    let path = file.unwrap_or_else(|| "src/styles.css".to_string());

    if watch {
        println!("Watching for changes...");
        Ok(())
    } else {
        println!("Checking {}", path);
        let mut css_file = fs::File::open(&path)?;
        let mut contents = String::new();
        css_file.read_to_string(&mut contents)?;



        Ok(())
    }
}
